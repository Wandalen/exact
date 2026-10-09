//! Splitting a value into equal parts under a chosen rounding mode, giving
//! the remainder ("dust") an explicit destination instead of letting it
//! silently vanish into truncation.
//!
//! Tier 3, depending on `exact_kind` for the conserved value types and
//! `exact_round` for [`exact_round::round_div`] — the same sign-handling,
//! tie-breaking division `exact_ratio` and `exact_snap` already share,
//! driven directly here rather than through `exact_ratio`.
//!
//! # Disclosed deviations from the preferred design's own listing
//!
//! - **Depends on `exact_round` directly, not `exact_ratio`.** The doc lists
//!   `exact_kind, exact_ratio` as this crate's dependencies, but the actual
//!   need is the integer-count, mode-driven division `exact_snap` already
//!   depends on `exact_round` directly for — not `exact_ratio`'s rational
//!   multiplier surface (`Ratio`, `mul_ratio`), none of which this crate
//!   uses. Depending on `exact_ratio` only to reach `round_div` indirectly
//!   would add an unused-feature dependency; `exact_snap` already
//!   establishes the direct-dependency precedent for this exact situation.
//! - **`parts` means an equal split count (`usize`), not a weighted list.**
//!   The doc's `dust_split(total, parts, mode, to)` does not say which; a
//!   count matches this family's existing `div_round`-style functions
//!   (divide by an integer count, not a ratio) and keeps "dust" its plain,
//!   documented meaning — the remainder of an equal integer division.
//! - **No `price_dust_split`.** `exact_ratio` already set this precedent by
//!   shipping `money_div_round`/`qty_div_round` with no `price_div_round` —
//!   "splitting a total among parties" and "splitting a holding into lots"
//!   are real operations for `Money` and `Quantity`; splitting a *price* into
//!   equal shares has no natural reading and no consumer anywhere in this
//!   codebase. Added if a real need ever names one.
//! - **`DustError` has no variant distinguishing a negative-quantity
//!   adjustment from an ordinary overflow.** The doc declares only
//!   `{EmptyParts, Remainder, Overflow}`. `exact_snap` already makes the same
//!   fold-everything-into-`Overflow` choice for its own `from_minor`
//!   reconstruction; this crate matches it.
//! - **The leftover is computed and redistributed at the raw minor-unit
//!   level, not through each kind's own checked arithmetic.** For `Qty`, an
//!   `Up`/`HalfEven` rounding mode can make the collective per-share
//!   allocation exceed the total by a small amount; correcting that at slot
//!   0 means *subtracting*, which `Qty::checked_add` cannot express (it only
//!   ever adds a non-negative quantity). Reconstructing slot 0 from its
//!   adjusted minor count via `from_minor` handles both directions uniformly
//!   and still refuses a result that would be genuinely negative — see
//!   `qty_dust_split_refuses_a_first_slot_that_would_go_negative_under_up_rounding`
//!   in this crate's tests.
//!
//! # Examples
//!
//! ```
//! use exact_dust::{ money_dust_split, DustTo };
//! use exact_kind::Money;
//! use exact_round::Rounding;
//!
//! let total = Money::from_minor( 11 ).unwrap();
//! let shares = money_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap();
//! assert_eq!( shares[ 0 ], Money::from_minor( 5 ).unwrap() ); // 2 + the 3-unit remainder
//! assert_eq!( shares[ 1 ], Money::from_minor( 2 ).unwrap() );
//! ```

use exact_kind::{ KindError, Money, Quantity };
use exact_round::{ RoundError, Rounding };

/// Where the remainder of an equal split goes.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DustTo
{
  /// Folded into the first part.
  First,
  /// Held back — not included in any output slot; query it separately via
  /// [`money_dust_remainder`]/[`qty_dust_remainder`].
  Sink,
  /// A nonzero remainder is refused outright.
  Reject,
}

/// Why a split could not be computed.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DustError
{
  /// Zero parts were requested.
  EmptyParts,
  /// The split did not divide evenly and [`DustTo::Reject`] or
  /// [`Rounding::Exact`] was asked.
  Remainder,
  /// An operation left the representable or declared range.
  Overflow,
}

impl core::fmt::Display for DustError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::EmptyParts => write!( f, "cannot split into zero parts" ),
      Self::Remainder => write!( f, "a remainder that DustTo::Reject or Rounding::Exact refuses" ),
      Self::Overflow => write!( f, "left the representable or declared range" ),
    }
  }
}

impl core::error::Error for DustError {}

fn round_error_to_dust_error( e : RoundError ) -> DustError
{
  match e
  {
    // Unreachable through this crate's own public API: every entry point
    // below checks `parts`/`out.len()` for zero before this is ever called,
    // so `round_div` never sees a zero divisor here. Mapped defensively
    // rather than asserted, the same defensive-but-unreachable pattern
    // `exact_kind::Decimal::checked_neg` already uses.
    RoundError::DivZero => DustError::EmptyParts,
    RoundError::Overflow => DustError::Overflow,
    // `Rounding::Exact` refuses a share with a remainder — the same fact
    // `DustTo::Reject` reports, so it is reported the same way.
    RoundError::Inexact => DustError::Remainder,
  }
}

/// The per-share minor count and the signed leftover against `total_minor`.
fn split_minor( total_minor : i64, parts : usize, mode : Rounding ) -> Result< ( i64, i64 ), DustError >
{
  if parts == 0
  {
    return Err( DustError::EmptyParts );
  }
  let parts_i64 = i64::try_from( parts ).map_err( | _ | DustError::Overflow )?;
  let share = exact_round::round_div( total_minor, parts_i64, mode ).map_err( round_error_to_dust_error )?;
  let allocated = share.checked_mul( parts_i64 ).ok_or( DustError::Overflow )?;
  let leftover = total_minor.checked_sub( allocated ).ok_or( DustError::Overflow )?;
  Ok( ( share, leftover ) )
}

/// Slot `i`'s minor count: `share`, with `leftover` folded into slot 0 when
/// `to` is [`DustTo::First`], and a nonzero `leftover` refused outright when
/// `to` is [`DustTo::Reject`]. One slot at a time, so the `_into` variants
/// need no buffer of their own.
fn slot_minor( share : i64, leftover : i64, to : DustTo, i : usize ) -> Result< i64, DustError >
{
  if leftover != 0 && matches!( to, DustTo::Reject )
  {
    return Err( DustError::Remainder );
  }
  if i == 0 && matches!( to, DustTo::First )
  {
    return share.checked_add( leftover ).ok_or( DustError::Overflow );
  }
  Ok( share )
}

/// Every slot of a `parts`-way split of `total_minor`, each built by `make` —
/// the one body behind [`money_dust_split`] and [`qty_dust_split`], which
/// differ only in the kind `make` builds.
fn split_with< T >
(
  total_minor : i64,
  parts : usize,
  mode : Rounding,
  to : DustTo,
  make : fn( i64 ) -> Result< T, KindError >,
) -> Result< Vec< T >, DustError >
{
  let ( share, leftover ) = split_minor( total_minor, parts, mode )?;
  ( 0 .. parts )
  .map( | i | make( slot_minor( share, leftover, to, i )? ).map_err( | _ | DustError::Overflow ) )
  .collect()
}

/// [`split_with`] written into `out` instead of a new `Vec`; `out.len()` is
/// the part count — the one body behind [`money_dust_split_into`] and
/// [`qty_dust_split_into`].
fn split_into_with< T >
(
  total_minor : i64,
  mode : Rounding,
  to : DustTo,
  out : &mut [ T ],
  make : fn( i64 ) -> Result< T, KindError >,
) -> Result< (), DustError >
{
  let ( share, leftover ) = split_minor( total_minor, out.len(), mode )?;
  // Fix(exact_dust_split_into_allocated): every slot's count used to be
  // collected into a `Vec` by `fill_minor` and then copied into `out` — one
  // heap allocation per call, against type/008's "does not allocate". Each
  // slot is now computed in place by `slot_minor`.
  //
  // Root cause: the `_into` variant reused the allocating `_split` helper.
  // Pitfall: a helper shared by an allocating and a non-allocating variant
  //   gives both the allocation, and the output is the same either way.
  for ( i, slot ) in out.iter_mut().enumerate()
  {
    let minor = slot_minor( share, leftover, to, i )?;
    *slot = make( minor ).map_err( | _ | DustError::Overflow )?;
  }
  Ok( () )
}

/// Split a money value into `parts` equal shares, rounding under `mode`,
/// sending the remainder to `to`.
///
/// # Errors
///
/// [`DustError::EmptyParts`] when `parts` is zero. [`DustError::Remainder`]
/// when [`DustTo::Reject`] or [`Rounding::Exact`] was asked and the split did
/// not divide evenly. [`DustError::Overflow`] on overflow or ceiling breach.
pub fn money_dust_split( total : Money, parts : usize, mode : Rounding, to : DustTo ) -> Result< Vec< Money >, DustError >
{
  split_with( total.minor(), parts, mode, to, Money::from_minor )
}

/// Non-allocating variant of [`money_dust_split`] — writes into `out` instead
/// of returning a `Vec`; `out.len()` is the part count.
///
/// # Errors
///
/// As [`money_dust_split`].
pub fn money_dust_split_into( total : Money, mode : Rounding, to : DustTo, out : &mut [ Money ] ) -> Result< (), DustError >
{
  split_into_with( total.minor(), mode, to, out, Money::from_minor )
}

/// The remainder a [`money_dust_split`] of `total` into `parts` under `mode`
/// would hold back, independent of where a [`DustTo`] would send it.
///
/// # Errors
///
/// As [`money_dust_split`]. [`DustError::Remainder`] only under
/// [`Rounding::Exact`] — this function reports the remainder, and refuses one
/// only when the mode itself forbids it.
pub fn money_dust_remainder( total : Money, parts : usize, mode : Rounding ) -> Result< i64, DustError >
{
  let ( _share, leftover ) = split_minor( total.minor(), parts, mode )?;
  Ok( leftover )
}

/// Split a quantity into `parts` equal shares, rounding under `mode`,
/// sending the remainder to `to`.
///
/// # Errors
///
/// As [`money_dust_split`].
pub fn qty_dust_split( total : Quantity, parts : usize, mode : Rounding, to : DustTo ) -> Result< Vec< Quantity >, DustError >
{
  split_with( total.minor(), parts, mode, to, Quantity::from_minor )
}

/// Non-allocating variant of [`qty_dust_split`].
///
/// # Errors
///
/// As [`money_dust_split_into`].
pub fn qty_dust_split_into( total : Quantity, mode : Rounding, to : DustTo, out : &mut [ Quantity ] ) -> Result< (), DustError >
{
  split_into_with( total.minor(), mode, to, out, Quantity::from_minor )
}

/// The remainder a [`qty_dust_split`] would hold back.
///
/// # Errors
///
/// As [`money_dust_remainder`].
pub fn qty_dust_remainder( total : Quantity, parts : usize, mode : Rounding ) -> Result< i64, DustError >
{
  let ( _share, leftover ) = split_minor( total.minor(), parts, mode )?;
  Ok( leftover )
}
