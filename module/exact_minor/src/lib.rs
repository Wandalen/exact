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
//! # What it provides
//!
//! - [`Minor`] — a count of minor units, a type of its own so it cannot be
//!   mixed up with any other `i64`. In and out via [`minor_from_i64`] and
//!   [`minor_to_i64`].
//! - Checked arithmetic that reports [`MinorError::Overflow`] (too big) or
//!   [`MinorError::Underflow`] (too small), and saturating arithmetic that
//!   clamps instead.
//! - `MinorWide` — the same at `i128` width, behind the `i128` feature.
//!
//! # Examples
//!
//! ```
//! use exact_minor::{ minor_checked_add, minor_from_i64, minor_to_i64 };
//!
//! let sum = minor_checked_add( minor_from_i64( 300_000 ), minor_from_i64( 200_000 ) ).unwrap();
//! assert_eq!( minor_to_i64( sum ), 500_000 );
//! ```

use core::fmt;

/// The backing integer width for every conserved value in the family.
///
/// Named exactly once, here, so a width change is one edit. Every other crate
/// in the family re-exports this alias rather than restating `i64`.
pub type Backing = i64;

/// A count of minor units — the family's base subunit type.
///
/// A distinct type rather than a bare `Backing`, so a count of minor units
/// cannot be confused with any other `i64`. The only ways in and out are
/// [`minor_from_i64`] and [`minor_to_i64`]; a bare `i64` is refused:
///
/// ```compile_fail
/// let _ = exact_minor::minor_checked_add( 1, 2 );
/// ```
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Minor( Backing );

/// Wrap a raw `i64` as a count of minor units.
#[ must_use ]
pub const fn minor_from_i64( v : i64 ) -> Minor
{
  Minor( v )
}

/// The raw `i64` a count of minor units holds.
#[ must_use ]
pub const fn minor_to_i64( m : Minor ) -> i64
{
  m.0
}

/// A count of minor units at twice the backing width, for magnitudes past `i64`.
///
/// Behind the `i128` feature — a flag on this crate, never a separate crate.
/// Any [`Minor`] widens into it without loss.
#[ cfg( feature = "i128" ) ]
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct MinorWide( pub i128 );

#[ cfg( feature = "i128" ) ]
impl From< Minor > for MinorWide
{
  fn from( m : Minor ) -> Self
  {
    Self( i128::from( m.0 ) )
  }
}

/// Narrowing back to the backing width, refused when the value does not fit.
#[ cfg( feature = "i128" ) ]
impl TryFrom< MinorWide > for Minor
{
  type Error = MinorError;

  fn try_from( w : MinorWide ) -> Result< Self, MinorError >
  {
    match Backing::try_from( w.0 )
    {
      Ok( v ) => Ok( Self( v ) ),
      Err( _ ) if w.0 > 0 => Err( MinorError::Overflow { operation : "narrow" } ),
      Err( _ ) => Err( MinorError::Underflow { operation : "narrow" } ),
    }
  }
}

/// Zero, in wide minor units.
#[ cfg( feature = "i128" ) ]
#[ must_use ]
pub const fn minor_wide_zero() -> MinorWide
{
  MinorWide( 0 )
}

/// Whether a wide count of minor units is exactly zero.
#[ cfg( feature = "i128" ) ]
#[ must_use ]
pub const fn minor_wide_is_zero( m : MinorWide ) -> bool
{
  m.0 == 0
}

/// Add two wide counts of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] when the sum rises above `i128::MAX`,
/// [`MinorError::Underflow`] when it falls below `i128::MIN`.
#[ cfg( feature = "i128" ) ]
pub const fn minor_wide_checked_add( a : MinorWide, b : MinorWide ) -> Result< MinorWide, MinorError >
{
  match a.0.checked_add( b.0 )
  {
    Some( sum ) => Ok( MinorWide( sum ) ),
    None if b.0 > 0 => Err( MinorError::Overflow { operation : "add" } ),
    None => Err( MinorError::Underflow { operation : "add" } ),
  }
}

/// Subtract two wide counts of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] when the difference rises above `i128::MAX`,
/// [`MinorError::Underflow`] when it falls below `i128::MIN`.
#[ cfg( feature = "i128" ) ]
pub const fn minor_wide_checked_sub( a : MinorWide, b : MinorWide ) -> Result< MinorWide, MinorError >
{
  match a.0.checked_sub( b.0 )
  {
    Some( diff ) => Ok( MinorWide( diff ) ),
    None if b.0 < 0 => Err( MinorError::Overflow { operation : "sub" } ),
    None => Err( MinorError::Underflow { operation : "sub" } ),
  }
}

/// Negate a wide count of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] on the one value that cannot negate, `i128::MIN`.
#[ cfg( feature = "i128" ) ]
pub const fn minor_wide_checked_neg( a : MinorWide ) -> Result< MinorWide, MinorError >
{
  match a.0.checked_neg()
  {
    Some( neg ) => Ok( MinorWide( neg ) ),
    None => Err( MinorError::Overflow { operation : "neg" } ),
  }
}

/// Add two wide counts of minor units, clamping to `i128`'s bounds rather than failing.
#[ cfg( feature = "i128" ) ]
#[ must_use ]
pub const fn minor_wide_saturating_add( a : MinorWide, b : MinorWide ) -> MinorWide
{
  MinorWide( a.0.saturating_add( b.0 ) )
}

/// Subtract two wide counts of minor units, clamping to `i128`'s bounds rather than failing.
#[ cfg( feature = "i128" ) ]
#[ must_use ]
pub const fn minor_wide_saturating_sub( a : MinorWide, b : MinorWide ) -> MinorWide
{
  MinorWide( a.0.saturating_sub( b.0 ) )
}

/// Why a checked operation could not be completed.
///
/// Two variants, one per direction: a result above `Backing::MAX` is an
/// overflow, a result below `Backing::MIN` an underflow, so an investigation
/// knows which bound was crossed.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum MinorError
{
  /// The result would have been above the largest value the backing width holds.
  Overflow
  {
    /// Which operation — `add`, `sub`, `neg`.
    operation : &'static str,
  },
  /// The result would have been below the smallest value the backing width holds.
  Underflow
  {
    /// Which operation — `add`, `sub`.
    operation : &'static str,
  },
}

impl fmt::Display for MinorError
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Self::Overflow { operation } => write!( f, "{operation} rose above the representable range" ),
      Self::Underflow { operation } => write!( f, "{operation} fell below the representable range" ),
    }
  }
}

impl core::error::Error for MinorError {}

/// Zero, in minor units.
#[ must_use ]
pub const fn minor_zero() -> Minor
{
  Minor( 0 )
}

/// Whether a count of minor units is exactly zero.
#[ must_use ]
pub const fn minor_is_zero( m : Minor ) -> bool
{
  m.0 == 0
}

/// Add two counts of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] when the sum rises above the backing width,
/// [`MinorError::Underflow`] when it falls below it.
pub const fn minor_checked_add( a : Minor, b : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_add( b.0 )
  {
    Some( sum ) => Ok( Minor( sum ) ),
    None if b.0 > 0 => Err( MinorError::Overflow { operation : "add" } ),
    None => Err( MinorError::Underflow { operation : "add" } ),
  }
}

/// Subtract two counts of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] when the difference rises above the backing width,
/// [`MinorError::Underflow`] when it falls below it.
pub const fn minor_checked_sub( a : Minor, b : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_sub( b.0 )
  {
    Some( diff ) => Ok( Minor( diff ) ),
    None if b.0 < 0 => Err( MinorError::Overflow { operation : "sub" } ),
    None => Err( MinorError::Underflow { operation : "sub" } ),
  }
}

/// Negate a count of minor units.
///
/// # Errors
///
/// [`MinorError::Overflow`] on the one backing value that cannot negate,
/// `Backing::MIN`.
pub const fn minor_checked_neg( a : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_neg()
  {
    Some( neg ) => Ok( Minor( neg ) ),
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
pub const fn minor_saturating_add( a : Minor, b : Minor ) -> Minor
{
  Minor( a.0.saturating_add( b.0 ) )
}

/// Subtract two counts of minor units, clamping to the backing width's own
/// bounds rather than failing.
#[ must_use ]
pub const fn minor_saturating_sub( a : Minor, b : Minor ) -> Minor
{
  Minor( a.0.saturating_sub( b.0 ) )
}
