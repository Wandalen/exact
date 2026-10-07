//! A rational multiplier (`Ratio`) and mode-driven integer division, per kind.
//!
//! Tier 2, depending on `exact_kind` for the conserved value types and
//! `exact_round` for the rounding modes every multiply and `div_round`-family
//! function takes, and for [`exact_round::round_div`] and its `i128` twin
//! [`exact_round::round_div_wide`] — the sign-handling, tie-breaking division
//! logic, shared with `exact_snap` rather than duplicated here.
//!
//! Net-new: no real precedent exists for either operation. Every multiply
//! here widens to `i128` before dividing, per this family's own documented
//! range budget — a multiply before a divide needs the wider type even when
//! every operand and the result fit the narrower one.
//!
//! # Disclosed deviations from the preferred design's own type listing
//!
//! - **`RatioError` carries no `ScaleMismatch` or `BadRounding`.**
//!   `ScaleMismatch` is unreachable for the same reason every other crate in
//!   this family already drops it — two different `SCALE` values are two
//!   different Rust types, caught at compile time. `BadRounding` is
//!   likewise unreachable: [`exact_round::Rounding`] is a closed seven-variant
//!   enum, and every value of it is already a valid rounding mode — there is
//!   no way to construct an invalid one through the public API for this
//!   error to report. In its place, `RatioError::Negative` carries the one
//!   real failure the doc's listing missed: a `Qty`-kind multiply or divide
//!   whose result would be negative.
//! - **A `Ratio`'s denominator is always stored positive.** `ratio_new`
//!   accepts a negative denominator and normalizes it by negating both
//!   fields — a negative ratio is conventionally a negative numerator over a
//!   positive denominator, and keeping the stored denominator positive means
//!   every later rounding calculation needs only one sign case instead of
//!   two.
//!
//! # Examples
//!
//! ```
//! use exact_kind::Money;
//! use exact_ratio::{ ratio_new, money_mul_ratio, money_div_round };
//! use exact_round::Rounding;
//!
//! let half = ratio_new( 1, 2 ).unwrap();
//! let v = Money::parse( "10" ).unwrap();
//! assert_eq!( money_mul_ratio( v, half, Rounding::HalfEven ).unwrap(), Money::parse( "5" ).unwrap() );
//! assert_eq!( money_div_round( v, 4, Rounding::HalfEven ).unwrap(), Money::parse( "2.5" ).unwrap() );
//! ```

use exact_kind::{ KindError, Money, Price, Quantity };
use exact_round::Rounding;

/// Why a ratio could not be constructed, or an operation could not be completed.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum RatioError
{
  /// A zero denominator was supplied.
  DivZero,
  /// An operation left the representable or declared range.
  Overflow,
  /// The result would have been below zero, for a kind that refuses it.
  Negative
  {
    /// The count of minor units the operation would have produced.
    minor : i64,
  },
}

impl core::fmt::Display for RatioError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::DivZero => write!( f, "a zero denominator was supplied" ),
      Self::Overflow => write!( f, "left the representable or declared range" ),
      Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
    }
  }
}

impl core::error::Error for RatioError {}

fn kind_error_to_ratio_error( e : KindError ) -> RatioError
{
  match e
  {
    KindError::Negative { minor } => RatioError::Negative { minor },
    _ => RatioError::Overflow,
  }
}

/// A rational multiplier `n / d`, with `d` always stored positive.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub struct Ratio
{
  n : i64,
  d : i64,
}

impl Ratio
{
  /// The numerator.
  #[ must_use ]
  pub const fn n( self ) -> i64
  {
    self.n
  }

  /// The denominator — always strictly positive.
  #[ must_use ]
  pub const fn d( self ) -> i64
  {
    self.d
  }
}

/// Build a ratio, refusing a zero denominator.
///
/// A negative denominator is accepted and normalized: `n / d` with `d < 0`
/// is stored as `-n / -d`, the conventional form, so every later rounding
/// calculation can assume a positive denominator.
///
/// # Errors
///
/// [`RatioError::DivZero`] when `d` is zero, [`RatioError::Overflow`] when
/// normalizing a negative denominator would overflow (only reachable at
/// `i64::MIN`).
pub const fn ratio_new( n : i64, d : i64 ) -> Result< Ratio, RatioError >
{
  if d == 0
  {
    return Err( RatioError::DivZero );
  }
  if d < 0
  {
    let Some( neg_n ) = n.checked_neg() else { return Err( RatioError::Overflow ) };
    let Some( neg_d ) = d.checked_neg() else { return Err( RatioError::Overflow ) };
    return Ok( Ratio { n : neg_n, d : neg_d } );
  }
  Ok( Ratio { n, d } )
}

fn mul_ratio_minor( minor : i64, r : Ratio, rounding : Rounding ) -> Result< i64, RatioError >
{
  // Fix(exact_ratio_mul_ratio_truncated_in_every_mode): the widened product
  // was divided with a bare `/`, so `7 × 1/2` gave 3 whatever the caller
  // wanted; it now divides through `round_div_wide` with the caller's mode.
  //
  // Root cause: `i128`'s `/` used as if it were a rounding division.
  // Pitfall: integer `/` always truncates toward zero — a division whose
  //   remainder matters has to name its rounding mode.
  let wide = i128::from( minor ) * i128::from( r.n );
  let divided = exact_round::round_div_wide( wide, i128::from( r.d ), rounding )
  .map_err( | _ | RatioError::Overflow )?;
  i64::try_from( divided ).map_err( | _ | RatioError::Overflow )
}

/// Multiply a money value by `n / d`, rounding the result per `rounding`.
///
/// # Errors
///
/// [`RatioError::Overflow`] when the widened product or the result leaves
/// the representable or declared range.
pub fn money_mul_ratio( v : Money, r : Ratio, rounding : Rounding ) -> Result< Money, RatioError >
{
  let minor = mul_ratio_minor( v.minor(), r, rounding )?;
  Money::from_minor( minor ).map_err( kind_error_to_ratio_error )
}

/// Multiply a quantity by `n / d`, rounding the result per `rounding`.
///
/// # Errors
///
/// [`RatioError::Negative`] when a negative-numerator ratio rounds the result below zero: at or below
/// -1 minor unit in every mode; within one of zero, `Down`/`AwayFromZero` always, `HalfEven`/`HalfDown`
/// past half a minor unit, `HalfUp` at half, `Up`/`TowardZero` never. [`RatioError::Overflow`] on overflow.
pub fn qty_mul_ratio( v : Quantity, r : Ratio, rounding : Rounding ) -> Result< Quantity, RatioError >
{
  let minor = mul_ratio_minor( v.minor(), r, rounding )?;
  Quantity::from_minor( minor ).map_err( kind_error_to_ratio_error )
}

/// Multiply a price by `n / d`, rounding the result per `rounding`.
///
/// # Errors
///
/// As [`money_mul_ratio`].
pub fn price_mul_ratio( v : Price, r : Ratio, rounding : Rounding ) -> Result< Price, RatioError >
{
  let minor = mul_ratio_minor( v.minor(), r, rounding )?;
  Price::from_minor( minor ).map_err( kind_error_to_ratio_error )
}

fn div_round_minor( n : i64, d : i64, rounding : Rounding ) -> Result< i64, RatioError >
{
  exact_round::round_div( n, d, rounding ).map_err( | e | match e
  {
    exact_round::RoundError::DivZero => RatioError::DivZero,
    exact_round::RoundError::Overflow => RatioError::Overflow,
  } )
}

/// Divide a money value by `d`, rounding the remainder per `rounding`.
///
/// # Errors
///
/// [`RatioError::DivZero`] when `d` is zero. [`RatioError::Overflow`] on
/// overflow or ceiling breach.
pub fn money_div_round( v : Money, d : i64, rounding : Rounding ) -> Result< Money, RatioError >
{
  let minor = div_round_minor( v.minor(), d, rounding )?;
  Money::from_minor( minor ).map_err( kind_error_to_ratio_error )
}

/// Divide a quantity by `d`, rounding the remainder per `rounding`.
///
/// # Errors
///
/// [`RatioError::DivZero`] when `d` is zero. [`RatioError::Negative`] when
/// the rounded result would be below zero. [`RatioError::Overflow`] on
/// overflow or ceiling breach.
pub fn qty_div_round( v : Quantity, d : i64, rounding : Rounding ) -> Result< Quantity, RatioError >
{
  let minor = div_round_minor( v.minor(), d, rounding )?;
  Quantity::from_minor( minor ).map_err( kind_error_to_ratio_error )
}

/// The money a trade costs: `price × qty`, rounding the result per `rounding`.
///
/// A quantity is itself a ratio — its minor-unit count over one whole unit —
/// so this is [`price_mul_ratio`]'s widened multiply and rounded divide, with
/// the quantity as the ratio. A fractional quantity counts in full.
///
/// # Errors
///
/// [`RatioError::Overflow`] when the cost leaves the representable or
/// declared range.
pub fn price_mul_qty( price : Price, qty : Quantity, rounding : Rounding ) -> Result< Money, RatioError >
{
  // `Quantity` and `Money` share one scale, so one whole quantity is `Money::ONE_MINOR` minor units.
  let qty_as_ratio = ratio_new( qty.minor(), Money::ONE_MINOR )?;
  let minor = mul_ratio_minor( price.minor(), qty_as_ratio, rounding )?;
  Money::from_minor( minor ).map_err( kind_error_to_ratio_error )
}

// `price_mul_qty` counts one whole quantity as `Money::ONE_MINOR` minor units, which holds only
// while `Quantity` and `Money` share one scale — fail the build the moment they drift apart.
const _ : () = assert!( matches!( Quantity::from_int( 1 ), Ok( q ) if q.minor() == Money::ONE_MINOR ) );
