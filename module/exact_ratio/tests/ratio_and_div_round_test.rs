//! `Ratio` construction and normalization, the widened multiply, and every
//! rounding mode `div_round` supports.

use exact_kind::{ Money, Price, Quantity };
use exact_ratio::{ RatioError, money_div_round, money_mul_ratio, price_mul_qty, qty_mul_ratio, ratio_new };
use exact_round::Rounding;

/// A zero denominator is refused.
#[ test ]
fn ratio_new_refuses_a_zero_denominator()
{
  assert_eq!( ratio_new( 1, 0 ), Err( RatioError::DivZero ) );
}

/// A negative denominator is normalized to a positive one, numerator negated.
#[ test ]
fn ratio_new_normalizes_a_negative_denominator()
{
  let r = ratio_new( 3, -4 ).unwrap();
  assert_eq!( r.n(), -3 );
  assert_eq!( r.d(), 4 );
}

/// Multiplying by one half halves the value exactly.
#[ test ]
fn mul_ratio_by_one_half_halves_the_value()
{
  let v = Money::parse( "10" ).unwrap();
  let half = ratio_new( 1, 2 ).unwrap();
  assert_eq!( money_mul_ratio( v, half, Rounding::HalfEven ).unwrap(), Money::parse( "5" ).unwrap() );
}

/// A multiply whose intermediate product would overflow `i64` still succeeds,
/// because the product is computed in `i128` before the divide narrows it
/// back down — the motivating case the range budget's own NFR describes.
#[ test ]
fn mul_ratio_survives_an_intermediate_that_would_overflow_i64()
{
  let r = ratio_new( 1_000_000, 1_000_000 ).unwrap(); // identity, but the product alone overflows i64
  assert_eq!( money_mul_ratio( Money::MAX, r, Rounding::HalfEven ).unwrap(), Money::MAX );
}

/// Multiplying the ceiling value by a ratio greater than one leaves the
/// declared range — `RatioError::Overflow`, distinct from the intermediate-
/// product case above, which the `i128` widening successfully absorbs.
#[ test ]
fn mul_ratio_reports_overflow_when_the_result_leaves_the_declared_range()
{
  let doubling = ratio_new( 2, 1 ).unwrap();
  assert_eq!( money_mul_ratio( Money::MAX, doubling, Rounding::HalfEven ), Err( RatioError::Overflow ) );
}

/// A negative-numerator ratio taking a quantity below zero is refused, not
/// wrapped — `RatioError::Negative`, not `RatioError::Overflow`.
#[ test ]
fn qty_mul_ratio_by_a_negative_ratio_is_refused_as_negative()
{
  let v = Quantity::from_int( 5 ).unwrap();
  let minus_one = ratio_new( -1, 1 ).unwrap();
  assert!( matches!( qty_mul_ratio( v, minus_one, Rounding::HalfEven ), Err( RatioError::Negative { .. } ) ) );
}

// Every case below divides a raw minor-unit count directly (via
// `from_minor`, not `from_int`) so the remainder is exactly the one under
// test — a whole-unit value's minor count is already a multiple of 10⁶ and
// divides evenly by most small integers, which would hide the rounding
// behaviour these tests exist to pin down.

/// `Down` rounds toward negative infinity at both signs.
#[ test ]
fn div_round_down_rounds_toward_negative_infinity()
{
  let seven = Money::from_minor( 7 ).unwrap();
  let minus_seven = Money::from_minor( -7 ).unwrap();
  assert_eq!( money_div_round( seven, 2, Rounding::Down ).unwrap().minor(), 3 );
  assert_eq!( money_div_round( minus_seven, 2, Rounding::Down ).unwrap().minor(), -4 );
}

/// `Up` rounds toward positive infinity at both signs.
#[ test ]
fn div_round_up_rounds_toward_positive_infinity()
{
  let seven = Money::from_minor( 7 ).unwrap();
  let minus_seven = Money::from_minor( -7 ).unwrap();
  assert_eq!( money_div_round( seven, 2, Rounding::Up ).unwrap().minor(), 4 );
  assert_eq!( money_div_round( minus_seven, 2, Rounding::Up ).unwrap().minor(), -3 );
}

/// `HalfEven` rounds an exact tie to the even neighbour, both directions.
#[ test ]
fn div_round_half_even_rounds_an_exact_tie_to_even()
{
  // 7 / 2 = 3.5 — ties to 4 ( even neighbour of {3, 4} ).
  let seven = Money::from_minor( 7 ).unwrap();
  assert_eq!( money_div_round( seven, 2, Rounding::HalfEven ).unwrap().minor(), 4 );

  // -7 / 2 = -3.5 — ties to -4 ( even neighbour of {-3, -4} ).
  let minus_seven = Money::from_minor( -7 ).unwrap();
  assert_eq!( money_div_round( minus_seven, 2, Rounding::HalfEven ).unwrap().minor(), -4 );

  // 5 / 2 = 2.5 — ties to 2 ( even neighbour of {2, 3} ).
  let five = Money::from_minor( 5 ).unwrap();
  assert_eq!( money_div_round( five, 2, Rounding::HalfEven ).unwrap().minor(), 2 );
}

/// `HalfEven` on a non-tying remainder rounds to the nearer neighbour.
#[ test ]
fn div_round_half_even_rounds_a_non_tie_to_the_nearest_neighbour()
{
  // 10 / 4 = 2.5 exactly -> ties to 2 (even).
  let ten = Money::from_minor( 10 ).unwrap();
  assert_eq!( money_div_round( ten, 4, Rounding::HalfEven ).unwrap().minor(), 2 );

  // 11 / 4 = 2.75 -> nearer to 3, not a tie.
  let eleven = Money::from_minor( 11 ).unwrap();
  assert_eq!( money_div_round( eleven, 4, Rounding::HalfEven ).unwrap().minor(), 3 );
}

/// A zero divisor is refused by `div_round`, same as by `ratio_new`.
#[ test ]
fn div_round_refuses_a_zero_divisor()
{
  let v = Money::from_int( 1 ).unwrap();
  assert_eq!( money_div_round( v, 0, Rounding::Down ), Err( RatioError::DivZero ) );
}

/// Dividing exactly, with no remainder, agrees across every rounding mode.
#[ test ]
fn an_exact_division_agrees_across_every_rounding_mode()
{
  let eight = Money::from_minor( 8 ).unwrap();
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven ]
  {
    assert_eq!( money_div_round( eight, 4, mode ).unwrap().minor(), 2 );
  }
}

/// A product that falls between two minor units is rounded the way the caller
/// asked — not silently cut toward zero, which every mode used to get.
#[ test ]
fn mul_ratio_rounds_per_the_callers_mode()
{
  let half = ratio_new( 1, 2 ).unwrap();
  let seven = Money::from_minor( 7 ).unwrap(); //  7 minor units × 1/2 =  3.5
  let minus_seven = Money::from_minor( -7 ).unwrap(); // -7 minor units × 1/2 = -3.5
  assert_eq!( money_mul_ratio( seven, half, Rounding::Down ).unwrap().minor(), 3 );
  assert_eq!( money_mul_ratio( seven, half, Rounding::Up ).unwrap().minor(), 4 );
  assert_eq!( money_mul_ratio( seven, half, Rounding::HalfEven ).unwrap().minor(), 4 );
  assert_eq!( money_mul_ratio( minus_seven, half, Rounding::Down ).unwrap().minor(), -4 );
  assert_eq!( money_mul_ratio( minus_seven, half, Rounding::Up ).unwrap().minor(), -3 );
  assert_eq!( money_mul_ratio( minus_seven, half, Rounding::HalfEven ).unwrap().minor(), -4 );
}

/// Price × quantity is the money a trade costs — a fractional quantity counts
/// in full, where `price.checked_mul_int( qty.whole() )` would drop the `.5`.
#[ test ]
fn price_mul_qty_is_the_cost_of_a_trade()
{
  let price = Price::parse( "1.25" ).unwrap();
  let qty = Quantity::parse( "4.5" ).unwrap();
  assert_eq!( price_mul_qty( price, qty, Rounding::HalfEven ).unwrap(), Money::parse( "5.625" ).unwrap() );
}

/// A cost finer than one minor unit is rounded per the caller's mode.
#[ test ]
fn price_mul_qty_rounds_a_cost_finer_than_one_minor_unit()
{
  let price = Price::parse( "0.000001" ).unwrap(); // one minor unit
  let qty = Quantity::parse( "0.5" ).unwrap(); // half a unit: the cost is half a minor unit
  assert_eq!( price_mul_qty( price, qty, Rounding::Down ).unwrap().minor(), 0 );
  assert_eq!( price_mul_qty( price, qty, Rounding::Up ).unwrap().minor(), 1 );
  assert_eq!( price_mul_qty( price, qty, Rounding::HalfEven ).unwrap().minor(), 0 ); // a tie goes to even
}

/// A cost past the declared ceiling is refused, not wrapped — even at both
/// operands' own ceilings, where the product overflows `i64` by far.
#[ test ]
fn price_mul_qty_refuses_a_cost_past_the_ceiling()
{
  assert_eq!( price_mul_qty( Price::MAX, Quantity::MAX, Rounding::HalfEven ), Err( RatioError::Overflow ) );
  let million = Price::parse( "1000000" ).unwrap();
  let million_units = Quantity::from_int( 1_000_000 ).unwrap(); // cost 10^12, past the 9 × 10^9 ceiling
  assert_eq!( price_mul_qty( million, million_units, Rounding::HalfEven ), Err( RatioError::Overflow ) );
}
