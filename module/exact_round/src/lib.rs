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
//! [`round_div`] and [`round_div_wide`] live here rather than in
//! `exact_ratio` or `exact_snap` individually, even though the preferred
//! design does not list them under this crate's own name: those tier-2
//! crates need "divide an integer by another, applying a rounding mode to
//! the remainder," and they already depend on this crate for [`Rounding`]
//! itself. Giving each of them its own private copy of the same
//! sign-handling and tie-breaking logic would be exactly the duplication
//! this family's own hygiene rules forbid; owning it once here, where every
//! consumer already has an edge, avoids it without adding a new dependency
//! edge to any of them. [`round_div_wide`] is the `i128` form, for
//! `exact_ratio`'s multiply-before-divide, whose product no `i64` can hold.
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
  /// The quotient does not fit the integer type — only reachable dividing
  /// the type's minimum value by `-1`.
  Overflow,
}

impl core::fmt::Display for RoundError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::DivZero => write!( f, "a zero divisor was supplied" ),
      Self::Overflow => write!( f, "the quotient does not fit the integer type" ),
    }
  }
}

impl core::error::Error for RoundError {}

/// Divide `n` by `d`, applying `rounding` to a nonzero remainder.
///
/// The `i64` form of [`round_div_wide`]: every `i64` fits an `i128`, so the
/// rounding rules are implemented once, there, and the result is narrowed back.
///
/// # Errors
///
/// [`RoundError::DivZero`] when `d` is zero. [`RoundError::Overflow`] when
/// the quotient does not fit an `i64` — only `i64::MIN / -1`.
pub const fn round_div( n : i64, d : i64, rounding : Rounding ) -> Result< i64, RoundError >
{
  match round_div_wide( n as i128, d as i128, rounding )
  {
    Ok( q ) if q >= i64::MIN as i128 && q <= i64::MAX as i128 => Ok( q as i64 ),
    Ok( _ ) => Err( RoundError::Overflow ),
    Err( e ) => Err( e ),
  }
}

/// Divide `n` by `d` over `i128`, applying `rounding` to a nonzero remainder.
///
/// The one implementation of the rounding rules: [`round_div`] calls it, and
/// `exact_ratio` calls it directly for a product of two `i64` values, which no
/// `i64` can hold. The operands keep their signs — the rounding direction
/// comes from the signs of the remainder and the divisor — so a minimum-value
/// operand rounds like any other.
///
/// # Errors
///
/// [`RoundError::DivZero`] when `d` is zero. [`RoundError::Overflow`] when
/// the quotient does not fit an `i128` — only `i128::MIN / -1`.
pub const fn round_div_wide( n : i128, d : i128, rounding : Rounding ) -> Result< i128, RoundError >
{
  if d == 0
  {
    return Err( RoundError::DivZero );
  }
  // Fix(exact_round_minimum_value_refused): a negative divisor used to be
  // handled by negating both operands first, which has no result for the
  // type's `MIN` (`i64::MIN` in `round_div`, `i128::MIN` here), so `(0, MIN)`,
  // `(1, MIN)`, `(MIN, -2)` and `(MIN, MIN)` returned `Overflow` though each
  // quotient fits. The operands now keep their signs; only the quotient can overflow.
  //
  // Root cause: `checked_neg` on an operand, where only the quotient can overflow.
  // Pitfall: two's-complement `MIN` has no positive counterpart — normalising
  //   signs by negation fails exactly at the edge a test grid rarely reaches.
  // Truncates toward zero; `MIN / -1` is the one quotient with no representable result.
  let Some( q ) = n.checked_div( d ) else { return Err( RoundError::Overflow ) };
  let r = n % d;
  if r == 0
  {
    return Ok( q );
  }

  // The exact quotient lies strictly between `q` and its neighbour one step
  // further from zero: below `q` when the remainder and divisor differ in sign.
  let exact_is_below = ( r < 0 ) != ( d < 0 );
  let step_toward_exact = match rounding
  {
    Rounding::Down => exact_is_below,
    Rounding::Up => !exact_is_below,
    Rounding::HalfEven =>
    {
      // `u128`: twice a remainder just below `i128::MAX` would not fit `i128`.
      let twice_r = r.unsigned_abs() * 2;
      let d_abs = d.unsigned_abs();
      twice_r > d_abs || ( twice_r == d_abs && q % 2 != 0 )
    }
  };
  if !step_toward_exact
  {
    return Ok( q );
  }
  // Cannot overflow: a nonzero remainder needs `|d| >= 2`, so `|q| <= |n| / 2`.
  Ok( if exact_is_below { q - 1 } else { q + 1 } )
}
