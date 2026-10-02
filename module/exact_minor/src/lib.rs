//! The raw subunit integer: checked and saturating arithmetic over the
//! backing width, with no scale or kind attached.
//!
//! Tier 0 of this family's fifteen crates, alongside `exact_scale` and
//! `exact_round`. It owns the backing integer width for the whole family and
//! depends on nothing — the same root role `exact_decimal` held before this
//! family was split along its preferred fifteen-crate boundaries.
//!
//! This crate answers one question only: can two counts of minor units be
//! added, subtracted, or negated within the backing width? It has no opinion
//! on scale (`exact_scale`'s concern) or on which kind of value is being
//! counted (`exact_kind`'s concern).
//!
//! # Disclosed deviations from the preferred design's own type listing
//!
//! - **No `Minor` newtype.** The preferred design names a `Minor(i64)`
//!   struct with `minor_from_i64`/`minor_to_i64` conversions. This crate
//!   exposes the backing width directly as [`Backing`] instead — a bare
//!   `pub type Backing = i64` alias — so there is nothing for those
//!   conversions to convert between, and neither exists.
//! - **No `MinorWide`.** The preferred design also names a feature-flagged
//!   `MinorWide(i128)` widening type. Every arithmetic function here already
//!   returns a `Result` on overflow rather than widening into a larger
//!   intermediate type, so no concrete call site has ever needed one.
//!
//! # Examples
//!
//! ```
//! use exact_minor::{ Backing, minor_checked_add };
//!
//! let sum : Result< Backing, _ > = minor_checked_add( 300_000, 200_000 );
//! assert_eq!( sum, Ok( 500_000 ) );
//! ```

use core::fmt;

/// The backing integer width for every conserved value in the family.
///
/// Named exactly once, here, so a width change is one edit. Every other crate
/// in the family re-exports this alias rather than restating `i64`.
pub type Backing = i64;

/// Why a checked operation could not be completed.
///
/// One variant, not a separate overflow/underflow split: `Backing`'s own
/// `checked_add`/`checked_sub`/`checked_neg` already report both directions
/// of range failure the same way, and inventing a sign-based distinction
/// those primitives do not make would be a check with no observable
/// behaviour behind it.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum MinorError
{
  /// An operation left the representable range of the backing width.
  Overflow
  {
    /// Which operation — `add`, `sub`, `neg`.
    operation : &'static str,
  },
}

impl fmt::Display for MinorError
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Self::Overflow { operation } => write!( f, "{operation} left the representable range" ),
    }
  }
}

impl core::error::Error for MinorError {}

/// Zero, in minor units.
#[ must_use ]
pub const fn minor_zero() -> Backing
{
  0
}

/// Whether a count of minor units is exactly zero.
#[ must_use ]
pub const fn minor_is_zero( m : Backing ) -> bool
{
  m == 0
}

/// Add two counts of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] when the sum leaves the backing width.
pub const fn minor_checked_add( a : Backing, b : Backing ) -> Result< Backing, MinorError >
{
  match a.checked_add( b )
  {
    Some( sum ) => Ok( sum ),
    None => Err( MinorError::Overflow { operation : "add" } ),
  }
}

/// Subtract two counts of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] when the difference leaves the backing width.
pub const fn minor_checked_sub( a : Backing, b : Backing ) -> Result< Backing, MinorError >
{
  match a.checked_sub( b )
  {
    Some( diff ) => Ok( diff ),
    None => Err( MinorError::Overflow { operation : "sub" } ),
  }
}

/// Negate a count of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] on the one backing value that cannot negate,
/// `Backing::MIN`.
pub const fn minor_checked_neg( a : Backing ) -> Result< Backing, MinorError >
{
  match a.checked_neg()
  {
    Some( neg ) => Ok( neg ),
    None => Err( MinorError::Overflow { operation : "neg" } ),
  }
}

/// Add two counts of minor units, clamping to the backing width's own bounds
/// rather than failing.
///
/// For call sites that have already decided a clamped result is an
/// acceptable answer to range failure — the checked variant stays the
/// default for call sites that have not.
#[ must_use ]
pub const fn minor_saturating_add( a : Backing, b : Backing ) -> Backing
{
  a.saturating_add( b )
}

/// Subtract two counts of minor units, clamping to the backing width's own
/// bounds rather than failing.
#[ must_use ]
pub const fn minor_saturating_sub( a : Backing, b : Backing ) -> Backing
{
  a.saturating_sub( b )
}
