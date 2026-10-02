//! Rounding modes and the family's default policy.
//!
//! Tier 0 of this family's fifteen crates, alongside `exact_minor` and
//! `exact_scale`. A sibling root with no edges to either — it defines the
//! rounding policy applied wherever a division or a snap-to-grid must land
//! on a representable value, independent of which backing or scale that
//! value is expressed in.
//!
//! Net-new: the family's prior shape (the 5 real crates this one is drawn
//! from) never offered more than one implicit rounding behaviour, so there
//! is no real-code precedent to port here — every item below is written
//! fresh against the preferred design's own crate specification.
//!
//! [`round_div`] lives here rather than in `exact_ratio` or `exact_snap`
//! individually, even though the preferred design does not list it under
//! this crate's own name: both of those tier-2 crates need "divide an
//! integer by another, applying a rounding mode to the remainder," and both
//! already depend on this crate for [`Rounding`] itself. Giving each of them
//! its own private copy of the same sign-handling and tie-breaking logic
//! would be exactly the duplication this family's own hygiene rules forbid;
//! owning it once here, where both consumers already have an edge, avoids
//! it without adding a new dependency edge to either.
//!
//! # Examples
//!
//! ```
//! use exact_round::{ Rounding, rounding_default, round_div };
//!
//! assert_eq!( rounding_default(), Rounding::HalfEven );
//! assert_eq!( round_div( 7, 2, Rounding::HalfEven ).unwrap(), 4 ); // 3.5 -> 4 (even)
//! ```

/// How a value that falls between two representable grid points is placed
/// onto one of them.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Rounding
{
  /// Round toward negative infinity — the floor.
  Down,

  /// Round toward positive infinity — the ceiling.
  Up,

  /// Round to the nearest grid point; on an exact tie, round to the point
  /// whose last digit is even.
  ///
  /// The tie-breaking rule a correctly-rounded decimal pipeline needs:
  /// rounding every tie the same direction biases a long sum of many
  /// rounded values, where biasing toward even cancels on average because
  /// ties land on an even last digit and an odd one equally often.
  HalfEven,
}

/// The family's default rounding policy where a call site states none.
///
/// `HalfEven` is the default because it is the only one of the three with
/// no directional bias over a long run of roundings — the property a
/// conserved-value family needs most, since a biased default would leak or
/// manufacture value on every unrounded remainder, silently, in one
/// direction, forever.
#[ must_use ]
pub const fn rounding_default() -> Rounding
{
  Rounding::HalfEven
}

/// A stable, human-readable name for a rounding mode.
///
/// For diagnostics and logs — never parsed back into a [`Rounding`], which
/// is why there is no corresponding `rounding_from_name`.
#[ must_use ]
pub const fn rounding_name( rounding : Rounding ) -> &'static str
{
  match rounding
  {
    Rounding::Down => "down",
    Rounding::Up => "up",
    Rounding::HalfEven => "half_even",
  }
}

/// Why a rounded division could not be completed.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum RoundError
{
  /// A zero divisor was supplied.
  DivZero,
  /// Normalizing a negative divisor, or adjusting the quotient by one,
  /// overflowed — only reachable at `i64::MIN`/`i64::MAX`.
  Overflow,
}

impl core::fmt::Display for RoundError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::DivZero => write!( f, "a zero divisor was supplied" ),
      Self::Overflow => write!( f, "adjusting the quotient for the chosen rounding mode overflowed" ),
    }
  }
}

impl core::error::Error for RoundError {}

/// Divide `n` by `d`, applying `rounding` to a nonzero remainder.
///
/// A negative `d` is accepted and normalized — `n / d` with `d < 0` is
/// computed as `-n / -d` — so every sign case below only has to handle a
/// positive divisor.
///
/// # Errors
///
/// [`RoundError::DivZero`] when `d` is zero. [`RoundError::Overflow`] when
/// normalizing a negative divisor, or adjusting the quotient by one,
/// overflows — only reachable at `i64::MIN`/`i64::MAX`.
pub const fn round_div( n : i64, d : i64, rounding : Rounding ) -> Result< i64, RoundError >
{
  if d == 0
  {
    return Err( RoundError::DivZero );
  }
  let ( n, d ) = if d < 0
  {
    let Some( neg_n ) = n.checked_neg() else { return Err( RoundError::Overflow ) };
    let Some( neg_d ) = d.checked_neg() else { return Err( RoundError::Overflow ) };
    ( neg_n, neg_d )
  }
  else
  {
    ( n, d )
  };

  let q = n / d;
  let r = n % d;
  if r == 0
  {
    return Ok( q );
  }

  match rounding
  {
    Rounding::Down =>
    {
      if r < 0
      {
        let Some( q ) = q.checked_sub( 1 ) else { return Err( RoundError::Overflow ) };
        Ok( q )
      }
      else
      {
        Ok( q )
      }
    }
    Rounding::Up =>
    {
      if r > 0
      {
        let Some( q ) = q.checked_add( 1 ) else { return Err( RoundError::Overflow ) };
        Ok( q )
      }
      else
      {
        Ok( q )
      }
    }
    Rounding::HalfEven =>
    {
      // `i128::from(_)` is not const-stable on this toolchain — `as` casts are.
      let twice_r_abs = ( r.unsigned_abs() as i128 ) * 2;
      let d_wide = d as i128;
      if twice_r_abs < d_wide
      {
        Ok( q )
      }
      else if twice_r_abs > d_wide || q % 2 != 0
      {
        if n < 0
        {
          let Some( q ) = q.checked_sub( 1 ) else { return Err( RoundError::Overflow ) };
          Ok( q )
        }
        else
        {
          let Some( q ) = q.checked_add( 1 ) else { return Err( RoundError::Overflow ) };
          Ok( q )
        }
      }
      else
      {
        Ok( q )
      }
    }
  }
}
