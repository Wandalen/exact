//! `Ratio` construction and normalization, the widened multiply, and every
//! rounding mode `div_round` supports.

use exact_kind::{ Money, Price, Quantity };
use exact_ratio::
{
  RatioError, money_div_round, money_mul_ratio, price_mul_qty, price_mul_ratio, qty_div_round, qty_mul_ratio,
  ratio_new,
};
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

/// A positive denominator is stored as given, numerator untouched.
#[ test ]
fn ratio_new_keeps_a_positive_denominator_as_given()
{
  let r = ratio_new( 3, 4 ).unwrap();
  assert_eq!( ( r.n(), r.d() ), ( 3, 4 ) );
}

/// Normalizing a negative denominator negates both fields, and `i64::MIN`
/// has no positive counterpart — either field at `i64::MIN` is refused.
#[ test ]
fn ratio_new_refuses_a_normalization_that_would_overflow()
{
  assert_eq!( ratio_new( 1, i64::MIN ), Err( RatioError::Overflow ) );
  assert_eq!( ratio_new( i64::MIN, -1 ), Err( RatioError::Overflow ) );
}

/// A price multiplies like money, each mode rounding its own way, at both
/// signs: `7 × 1/2 = 3.5` and `-7 × 1/2 = -3.5`.
#[ test ]
fn price_mul_ratio_rounds_per_the_callers_mode_at_both_signs()
{
  let half = ratio_new( 1, 2 ).unwrap();
  let seven = Price::from_minor( 7 ).unwrap();
  let minus_seven = Price::from_minor( -7 ).unwrap();
  assert_eq!( price_mul_ratio( seven, half, Rounding::Down ).unwrap().minor(), 3 );
  assert_eq!( price_mul_ratio( seven, half, Rounding::Up ).unwrap().minor(), 4 );
  assert_eq!( price_mul_ratio( seven, half, Rounding::HalfEven ).unwrap().minor(), 4 );
  assert_eq!( price_mul_ratio( minus_seven, half, Rounding::Down ).unwrap().minor(), -4 );
  assert_eq!( price_mul_ratio( minus_seven, half, Rounding::Up ).unwrap().minor(), -3 );
  assert_eq!( price_mul_ratio( minus_seven, half, Rounding::HalfEven ).unwrap().minor(), -4 );
}

/// A price pushed past its ceiling is refused, not wrapped.
#[ test ]
fn price_mul_ratio_refuses_a_result_past_the_ceiling()
{
  let double = ratio_new( 2, 1 ).unwrap();
  assert_eq!( price_mul_ratio( Price::MAX, double, Rounding::Down ), Err( RatioError::Overflow ) );
}

/// A negative ratio rounds with the sign handled correctly: `10 × -1/3 =
/// -3.33…`, whose floor is `-4` and ceiling `-3`.
#[ test ]
fn money_mul_ratio_by_a_negative_ratio_rounds_toward_the_named_infinity()
{
  let minus_third = ratio_new( -1, 3 ).unwrap();
  let ten = Money::from_minor( 10 ).unwrap();
  assert_eq!( money_mul_ratio( ten, minus_third, Rounding::Down ).unwrap().minor(), -4 );
  assert_eq!( money_mul_ratio( ten, minus_third, Rounding::Up ).unwrap().minor(), -3 );
  assert_eq!( money_mul_ratio( ten, minus_third, Rounding::HalfEven ).unwrap().minor(), -3 );
}

/// A zero ratio gives zero, whatever the value and mode.
#[ test ]
fn mul_ratio_by_zero_is_zero()
{
  let zero = ratio_new( 0, 5 ).unwrap();
  let v = Money::from_minor( -7 ).unwrap();
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven ]
  {
    assert_eq!( money_mul_ratio( v, zero, mode ).unwrap(), Money::ZERO );
  }
}

/// A quantity multiplies with each mode rounding its own way — `7 × 1/2 =
/// 3.5`, and `5 × 1/2 = 2.5`, a tie `HalfEven` sends to the even 2.
#[ test ]
fn qty_mul_ratio_rounds_per_the_callers_mode()
{
  let half = ratio_new( 1, 2 ).unwrap();
  let seven = Quantity::from_minor( 7 ).unwrap();
  assert_eq!( qty_mul_ratio( seven, half, Rounding::Down ).unwrap().minor(), 3 );
  assert_eq!( qty_mul_ratio( seven, half, Rounding::Up ).unwrap().minor(), 4 );
  assert_eq!( qty_mul_ratio( seven, half, Rounding::HalfEven ).unwrap().minor(), 4 );
  let five = Quantity::from_minor( 5 ).unwrap();
  assert_eq!( qty_mul_ratio( five, half, Rounding::HalfEven ).unwrap().minor(), 2 );
}

/// A quantity divides with each mode rounding its own way: `7 / 2 = 3.5`.
#[ test ]
fn qty_div_round_rounds_per_the_callers_mode()
{
  let seven = Quantity::from_minor( 7 ).unwrap();
  assert_eq!( qty_div_round( seven, 2, Rounding::Down ).unwrap().minor(), 3 );
  assert_eq!( qty_div_round( seven, 2, Rounding::Up ).unwrap().minor(), 4 );
  assert_eq!( qty_div_round( seven, 2, Rounding::HalfEven ).unwrap().minor(), 4 );
}

/// A zero divisor is refused for a quantity too.
#[ test ]
fn qty_div_round_refuses_a_zero_divisor()
{
  let v = Quantity::from_int( 1 ).unwrap();
  assert_eq!( qty_div_round( v, 0, Rounding::Down ), Err( RatioError::DivZero ) );
}

/// A negative divisor would make a quantity negative, which it refuses —
/// unless the rounded result is zero: `1 / -2 = -0.5` floors to `-1` but
/// ceils to `0`.
#[ test ]
fn qty_div_round_by_a_negative_divisor_is_refused_unless_it_rounds_to_zero()
{
  let ten = Quantity::from_minor( 10 ).unwrap();
  assert_eq!( qty_div_round( ten, -2, Rounding::Down ), Err( RatioError::Negative { minor : -5 } ) );
  let one = Quantity::from_minor( 1 ).unwrap();
  assert_eq!( qty_div_round( one, -2, Rounding::Down ), Err( RatioError::Negative { minor : -1 } ) );
  assert_eq!( qty_div_round( one, -2, Rounding::Up ).unwrap(), Quantity::ZERO );
}

/// A negative price — a debit-style quote — costs negative money, and a zero
/// quantity costs nothing.
#[ test ]
fn price_mul_qty_carries_a_negative_price_and_a_zero_quantity()
{
  let negative = Price::parse( "-4" ).unwrap();
  let qty = Quantity::parse( "2.5" ).unwrap();
  assert_eq!( price_mul_qty( negative, qty, Rounding::HalfEven ).unwrap(), Money::parse( "-10" ).unwrap() );
  assert_eq!( price_mul_qty( negative, Quantity::ZERO, Rounding::HalfEven ).unwrap(), Money::ZERO );
}

/// Every error renders a message naming its cause.
#[ test ]
fn every_ratio_error_renders_its_cause()
{
  assert_eq!( RatioError::DivZero.to_string(), "a zero denominator was supplied" );
  assert_eq!( RatioError::Overflow.to_string(), "left the representable or declared range" );
  assert_eq!(
    RatioError::Negative { minor : -5 }.to_string(),
    "-5 minor units is below zero, which this kind cannot hold"
  );
}

/// A negative ratio on a quantity is refused only when the chosen mode rounds
/// the product below zero: one minor unit × -1/3 is -0.33…, which `Down`
/// floors to -1 (refused) and `Up`/`HalfEven` take to 0 (accepted).
#[ test ]
fn qty_mul_ratio_by_a_sub_unit_negative_product_depends_on_the_mode()
{
  let one = Quantity::from_minor( 1 ).unwrap();
  let neg_third = ratio_new( -1, 3 ).unwrap();
  assert_eq!( qty_mul_ratio( one, neg_third, Rounding::Down ), Err( RatioError::Negative { minor : -1 } ) );
  assert_eq!( qty_mul_ratio( one, neg_third, Rounding::Up ).unwrap(), Quantity::ZERO );
  assert_eq!( qty_mul_ratio( one, neg_third, Rounding::HalfEven ).unwrap(), Quantity::ZERO );
}
