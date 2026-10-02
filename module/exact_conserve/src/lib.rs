//! Conservation auditing: does a set of postings sum to zero?
//!
//! Carries `exact_audit`'s whole-log auditor forward unchanged in behaviour
//! — [`Entry`], [`Report`], and [`verify`] still take a plain `i64`-amount
//! posting and compute an `i128`-widened net total, exactly as before — and
//! adds a typed per-kind convenience layer on top: [`money_conserve_into`]/
//! [`qty_conserve_into`] for folding one typed leg at a time via `exact_add`,
//! and [`money_sum_assert_zero`]/[`qty_sum_assert_zero`] for asserting a
//! whole typed slice conserves.
//!
//! # What conservation means here
//!
//! A log is a sequence of signed postings. Value is neither created nor
//! destroyed by a transfer, so every transfer contributes one credit and one
//! matching debit, and the whole log therefore sums to zero. A non-zero sum
//! is a *discrepancy*: value appeared or vanished between two postings.
//!
//! Per-account totals are deliberately not computed. An account's total is
//! its balance, and a non-zero balance is the normal state of an account,
//! not a finding — reporting balances alongside a conservation verdict would
//! put a column of expected non-zeros next to the one non-zero that means
//! something.
//!
//! # Widths
//!
//! Postings are `i64`, matching the family's backing width; [`verify`]'s and
//! the `sum_assert_zero` functions' accumulators are `i128`, strictly wider
//! — the fold is still checked, so even the length at which `i128` would run
//! out — somewhere past `2⁶⁴` maximal postings — returns an error rather
//! than wrapping.
//!
//! # Disclosed deviations from `exact_audit` and the preferred design
//!
//! - **No longer zero-dependency.** `exact_audit`'s own
//!   `docs/decisions/001_zero_dependency_by_contract.md` fixed its manifest
//!   at no `[dependencies]` at all, enforced by a manifest-reading test, so
//!   the auditor could never reach into this family's other value types.
//!   The preferred design's own dependency-tree edge
//!   (`exact_conserve → exact_add, exact_kind`) retires that Contract for
//!   this crate: the new typed layer genuinely needs `exact_kind`'s `Money`/
//!   `Quantity` and `exact_add`'s checked arithmetic. [`Entry`], [`Report`],
//!   and [`verify`] themselves still touch neither — they remain exactly as
//!   dependency-free *in their own logic* as before, carrying forward the
//!   original Contract's actual engineering value (an auditor testable
//!   against a log from any source, not only live in-process values) even
//!   though the crate's manifest as a whole is no longer empty. The
//!   manifest-reading test (`the_manifest_declares_no_dependencies`) is
//!   therefore dropped rather than ported — it tests a property this crate
//!   deliberately no longer holds.
//! - **`AuditError` is renamed `ConservationError`, per the preferred
//!   design**, and its single variant's shape changes with it:
//!   `AuditError::AccumulatorOverflow { at_entry }` becomes the doc's bare
//!   `ConservationError::Overflow` (no field) — the doc specifies this
//!   crate's error shape explicitly, unlike most others in this family that
//!   leave it to be inferred, so the position-tracking `at_entry` field is
//!   dropped rather than preserved as a deviation. A caller that needs to
//!   bisect a failing log can still do so externally.
//! - **`ConservationError::NotZero { got : i128 }`.** The doc does not say
//!   what type `got` carries. `i128` matches [`Report::discrepancy_minor`]'s
//!   own type and sidesteps a representation problem specific to `Quantity`:
//!   a non-negative kind cannot hold a negative leg, so there is no typed
//!   signed-quantity value a `got` field could carry for
//!   [`qty_sum_assert_zero`] — a raw minor-unit count is the only
//!   representation that works for both kinds uniformly.
//! - **`qty_sum_assert_zero`'s practical meaning is narrower than
//!   `money_sum_assert_zero`'s.** Every `Quantity` leg is individually
//!   non-negative, so their sum is zero only when every leg is
//!   [`exact_kind::Qty::ZERO`] — a real, if narrow, check (e.g. "nothing is
//!   left unaccounted after a full reconciliation"), not the general
//!   credit/debit conservation check `money_sum_assert_zero` performs.
//! - **`money_conserve_into`/`qty_conserve_into` are prefixed per kind**,
//!   matching this family's established convention (`exact_add`,
//!   `exact_ratio`, `exact_cmp`, …), rather than the doc's single generic
//!   `conserve_into(acc, leg)` — the same choice already made for
//!   `exact_dust`'s `money_dust_split`/`qty_dust_split`.

use exact_kind::{ KindError, Money, Quantity };

/// One posting in a transaction log.
///
/// Plain data, constructed by anyone, carrying no invariant of its own. The
/// signed amount is a count of minor units at whatever scale the log's
/// producer and consumer have agreed on; [`verify`] never interprets the
/// scale, because conservation is a property of the integers and holds at
/// every scale.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Entry
{
  /// The account the posting is against. Carried for reporting, never for
  /// arithmetic — see the module docs on why balances are not totalled.
  pub account : String,
  /// Signed minor units: positive credits the account, negative debits it.
  pub amount_minor : i64,
}

impl Entry
{
  /// Build a posting.
  #[ must_use ]
  pub fn new( account : impl Into< String >, amount_minor : i64 ) -> Self
  {
    Self { account : account.into(), amount_minor }
  }
}

/// Why a conservation check could not be completed, or found a discrepancy.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ConservationError
{
  /// A typed slice's sum was not exactly zero.
  NotZero
  {
    /// The actual signed sum, in minor units.
    got : i128,
  },
  /// The running total left the representable range.
  Overflow,
}

impl core::fmt::Display for ConservationError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::NotZero { got } => write!( f, "expected a zero sum, got {got} minor units" ),
      Self::Overflow => write!( f, "the running total left the representable range" ),
    }
  }
}

impl core::error::Error for ConservationError {}

fn kind_error_to_conservation_error( _e : KindError ) -> ConservationError
{
  ConservationError::Overflow
}

/// The outcome of auditing a log.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Report
{
  /// How many postings were folded.
  pub entries : usize,
  /// Their signed sum, in minor units. Zero is a balanced log.
  pub net_minor : i128,
}

impl Report
{
  /// Whether the log conserves value.
  ///
  /// Exact equality with zero, with no tolerance window. A tolerance is how
  /// an auditor comes to pass the only errors small enough to be worth
  /// hiding: an off-by-one-unit leak repeated across a million transactions
  /// is the failure mode this whole crate exists to make impossible, and it
  /// is invisible to any check that ignores single units.
  #[ must_use ]
  pub const fn is_balanced( &self ) -> bool
  {
    self.net_minor == 0
  }

  /// The discrepancy, in minor units — zero when balanced.
  ///
  /// Signed on purpose: the sign says whether value appeared or vanished,
  /// and those are different investigations.
  #[ must_use ]
  pub const fn discrepancy_minor( &self ) -> i128
  {
    self.net_minor
  }
}

impl core::fmt::Display for Report
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    if self.is_balanced()
    {
      write!( f, "balanced: entries {}, net 0", self.entries )
    }
    else
    {
      write!( f, "UNBALANCED: entries {}, net {} minor units", self.entries, self.net_minor )
    }
  }
}

/// Audit a log for conservation.
///
/// ```
/// use exact_conserve::{ Entry, verify };
///
/// let log = [ Entry::new( "hold", 1_000_000 ), Entry::new( "ship", -1_000_000 ) ];
/// assert!( verify( &log ).unwrap().is_balanced() );
///
/// let leaky = [ Entry::new( "hold", 1_000_000 ), Entry::new( "ship", -999_999 ) ];
/// assert_eq!( verify( &leaky ).unwrap().discrepancy_minor(), 1 );
/// ```
///
/// # Errors
///
/// [`ConservationError::Overflow`] if the running total leaves `i128`.
pub fn verify( entries : &[ Entry ] ) -> Result< Report, ConservationError >
{
  let mut net : i128 = 0;
  for entry in entries
  {
    net = net
    .checked_add( i128::from( entry.amount_minor ) )
    .ok_or( ConservationError::Overflow )?;
  }
  Ok( Report { entries : entries.len(), net_minor : net } )
}

/// Fold one more money leg into a running total, via `exact_add`'s own
/// checked arithmetic. Suitable for [`Iterator::try_fold`].
///
/// # Errors
///
/// [`ConservationError::Overflow`] on overflow or ceiling breach.
pub fn money_conserve_into( acc : Money, leg : Money ) -> Result< Money, ConservationError >
{
  exact_add::money_add( acc, leg ).map_err( kind_error_to_conservation_error )
}

/// Fold one more quantity leg into a running total, via `exact_add`'s own
/// checked arithmetic. Suitable for [`Iterator::try_fold`].
///
/// # Errors
///
/// [`ConservationError::Overflow`] on overflow or ceiling breach.
pub fn qty_conserve_into( acc : Quantity, leg : Quantity ) -> Result< Quantity, ConservationError >
{
  exact_add::qty_add( acc, leg ).map_err( kind_error_to_conservation_error )
}

/// Assert a slice of money legs sums to exactly zero.
///
/// # Errors
///
/// [`ConservationError::NotZero`] when the sum is not zero.
/// [`ConservationError::Overflow`] if the running total leaves `i128`.
pub fn money_sum_assert_zero( legs : &[ Money ] ) -> Result< (), ConservationError >
{
  let mut net : i128 = 0;
  for leg in legs
  {
    net = net.checked_add( i128::from( leg.minor() ) ).ok_or( ConservationError::Overflow )?;
  }
  if net == 0
  {
    Ok( () )
  }
  else
  {
    Err( ConservationError::NotZero { got : net } )
  }
}

/// Assert a slice of quantity legs sums to exactly zero.
///
/// Every `Quantity` is individually non-negative, so this holds only when
/// every leg is [`exact_kind::Qty::ZERO`] — narrower than
/// [`money_sum_assert_zero`]'s general conservation check, but the same
/// shape.
///
/// # Errors
///
/// [`ConservationError::NotZero`] when the sum is not zero.
/// [`ConservationError::Overflow`] if the running total leaves `i128`.
pub fn qty_sum_assert_zero( legs : &[ Quantity ] ) -> Result< (), ConservationError >
{
  let mut net : i128 = 0;
  for leg in legs
  {
    net = net.checked_add( i128::from( leg.minor() ) ).ok_or( ConservationError::Overflow )?;
  }
  if net == 0
  {
    Ok( () )
  }
  else
  {
    Err( ConservationError::NotZero { got : net } )
  }
}
