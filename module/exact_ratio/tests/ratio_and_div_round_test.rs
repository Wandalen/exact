//! `Ratio` construction and normalization, the widened multiply, and `div_round`
//! under `Down`, `Up` and `HalfEven`, with mode-independent cases under all eight,
//! and `Exact`'s refusal of a result that needs rounding.

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
  assert!
  (
    matches!( qty_mul_ratio( v, minus_one, Rounding::HalfEven ), Err( RatioError::Negative { .. } ) )
  );
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
  for mode in EVERY_MODE
  {
    assert_eq!( money_div_round( eight, 4, mode ).unwrap().minor(), 2 );
  }
}

/// A product that falls between two minor units is rounded the way the caller
/// asked — not silently cut toward zero, which every mode used to get.
///
/// Root Cause: `mul_ratio_minor` divided the widened product with a bare
/// `/`, which truncates toward zero, and took no rounding mode at all — so
/// `7 × 1/2` gave 3 and `-7 × 1/2` gave -3 whatever the caller needed.
///
/// Why Not Caught: the existing multiply tests used ratios that divide
/// evenly, and `price_mul_ratio` had no test at all; no product landed
/// between two minor units, where truncation and rounding disagree.
///
/// Fix Applied: the three `*_mul_ratio` functions take a `Rounding`, and
/// `mul_ratio_minor` divides through `exact_round::round_div_wide` with it.
///
/// Prevention: this test pins `Down`, `Up` and `HalfEven` on a positive and a negative
/// half-unit product, and `price_mul_ratio_rounds_per_the_callers_mode_at_both_signs`
/// and `qty_mul_ratio_rounds_per_the_callers_mode` below repeat it for the other kinds.
///
/// Pitfall: integer `/` always rounds toward zero — a division whose
/// remainder matters has to name its rounding mode.
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
  for mode in EVERY_MODE
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
  assert_eq!( RatioError::Inexact.to_string(), "the result needed rounding and Rounding::Exact was requested" );
}

/// A multiply under `Exact` that needs rounding is reported as `Inexact`, not
/// as `Overflow`.
///
/// Root Cause: `mul_ratio_minor` mapped every `RoundError` from
/// `round_div_wide` to `RatioError::Overflow` with `| _ |`. With only
/// `DivZero` (unreachable: a `Ratio`'s denominator is never zero) and
/// `Overflow` to map, that was harmless; with `Inexact` it would have told a
/// caller an in-range result had left the range.
///
/// Why Not Caught: `div_round_minor` matched every variant, so the compiler
/// flagged it when `RoundError` grew; the wildcard in `mul_ratio_minor` was
/// silent.
///
/// Fix Applied: both go through `round_error_to_ratio_error`, an exhaustive
/// `match`, so a new `RoundError` variant fails the build until it is mapped.
///
/// Prevention: this test pins `Inexact` through every multiply that reaches
/// `mul_ratio_minor`; it fails on the wildcard.
///
/// Pitfall: a `| _ |` error mapping is correct only for the variants that
/// exist the day it is written.
#[ test ]
fn a_multiply_needing_rounding_under_exact_is_inexact_not_overflow()
{
  let half = ratio_new( 1, 2 ).unwrap();
  let seven = Money::from_minor( 7 ).unwrap();
  assert_eq!( money_mul_ratio( seven, half, Rounding::Exact ), Err( RatioError::Inexact ) );
  assert_eq!( price_mul_ratio( Price::from_minor( 7 ).unwrap(), half, Rounding::Exact ), Err( RatioError::Inexact ) );
  assert_eq!( qty_mul_ratio( Quantity::from_minor( 7 ).unwrap(), half, Rounding::Exact ), Err( RatioError::Inexact ) );
  assert_eq!( money_mul_ratio( Money::from_minor( 8 ).unwrap(), half, Rounding::Exact ).unwrap().minor(), 4 );
}

/// A rounded division under `Exact` returns an exact quotient and refuses a
/// remainder, for both kinds.
#[ test ]
fn div_round_under_exact_refuses_a_remainder()
{
  assert_eq!( money_div_round( Money::from_minor( 7 ).unwrap(), 2, Rounding::Exact ), Err( RatioError::Inexact ) );
  assert_eq!( money_div_round( Money::from_minor( 8 ).unwrap(), -2, Rounding::Exact ).unwrap().minor(), -4 );
  assert_eq!( qty_div_round( Quantity::from_minor( 7 ).unwrap(), 2, Rounding::Exact ), Err( RatioError::Inexact ) );
  assert_eq!( qty_div_round( Quantity::from_minor( 9 ).unwrap(), 3, Rounding::Exact ).unwrap().minor(), 3 );
}

/// Under `Exact` a quantity's product is judged exact before it is judged
/// non-negative: a negative product that needs rounding is `Inexact`, and an
/// exact negative one is `Negative`.
#[ test ]
fn qty_mul_ratio_under_exact_checks_exactness_before_sign()
{
  let one = Quantity::from_minor( 1 ).unwrap();
  assert_eq!( qty_mul_ratio( one, ratio_new( -1, 3 ).unwrap(), Rounding::Exact ), Err( RatioError::Inexact ) );
  assert_eq!( qty_mul_ratio( one, ratio_new( -1, 1 ).unwrap(), Rounding::Exact ), Err( RatioError::Negative { minor : -1 } ) );
}

/// `price_mul_qty` under `Exact` is a settlement cost that is refused, never
/// rounded — the same answers the exchange's own `notional` gives: exact
/// costs come back exact, a cost finer than one minor unit is refused, and a
/// cost past the ceiling is still `Overflow`.
#[ test ]
fn price_mul_qty_under_exact_refuses_a_cost_that_needs_rounding()
{
  for ( price, qty, cost ) in [ ( "1.25", 4, "5" ), ( "0.000001", 1_000_000, "1" ), ( "3", 7, "21" ), ( "0.5", 3, "1.5" ) ]
  {
    let price = Price::parse( price ).unwrap();
    let qty = Quantity::from_int( qty ).unwrap();
    assert_eq!( price_mul_qty( price, qty, Rounding::Exact ).unwrap(), Money::parse( cost ).unwrap(), "{price} × {qty}" );
  }
  // One minor unit of price × one minor unit of quantity: twelve places on a six-place money.
  let dust = price_mul_qty( Price::from_minor( 1 ).unwrap(), Quantity::EPSILON, Rounding::Exact );
  assert_eq!( dust, Err( RatioError::Inexact ) );
  let half_unit = price_mul_qty( Price::from_minor( 1 ).unwrap(), Quantity::parse( "0.5" ).unwrap(), Rounding::Exact );
  assert_eq!( half_unit, Err( RatioError::Inexact ) );
  let past_ceiling = price_mul_qty( Price::parse( "1000000000" ).unwrap(), Quantity::from_int( 10 ).unwrap(), Rounding::Exact );
  assert_eq!( past_ceiling, Err( RatioError::Overflow ) );
}

/// A negative ratio on a quantity is refused whenever the rounded product is below zero; the mode
/// decides only within one minor unit of zero. One minor unit × -1/3 is -0.33… of a minor unit, which
/// only `Down` and `AwayFromZero` take to -1 (refused); one minor unit × -1/2 is an exact tie, which
/// `HalfUp` takes to -1 too; two minor units × -1/3 give -0.66…, past half of one, which only `Up` and
/// `TowardZero` still take to 0. A whole-unit product (1.5 units × -1/3) is refused in every mode.
#[ test ]
fn qty_mul_ratio_by_a_sub_unit_negative_product_depends_on_the_mode()
{
  use Rounding::*;
  let refused = Err( RatioError::Negative { minor : -1 } );
  let zero = Ok( Quantity::ZERO );
  let modes = [ Down, AwayFromZero, HalfEven, HalfDown, HalfUp, Up, TowardZero ];
  //  minor, n, d,   Down,    AwayFromZero, HalfEven, HalfDown, HalfUp,  Up,   TowardZero
  let cases =
  [
    ( 1, -1, 3, [ refused, refused, zero,    zero,    zero,    zero, zero ] ), // -0.33
    ( 1, -1, 2, [ refused, refused, zero,    zero,    refused, zero, zero ] ), // -0.5, a tie
    ( 2, -1, 3, [ refused, refused, refused, refused, refused, zero, zero ] ), // -0.67
  ];
  for ( minor, n, d, expected ) in cases
  {
    let v = Quantity::from_minor( minor ).unwrap();
    let r = ratio_new( n, d ).unwrap();
    for ( mode, want ) in modes.into_iter().zip( expected )
    {
      assert_eq!( qty_mul_ratio( v, r, mode ), want, "{minor} minor × {n}/{d} under {mode:?}" );
    }
  }
  let one_and_a_half = Quantity::parse( "1.5" ).unwrap();
  let neg_third = ratio_new( -1, 3 ).unwrap();
  let whole_refused = Err( RatioError::Negative { minor : -500_000 } );
  for mode in EVERY_MODE
  {
    assert_eq!( qty_mul_ratio( one_and_a_half, neg_third, mode ), whole_refused );
  }
}

/// Every rounding mode, for the tests that must hold under each of them.
const EVERY_MODE : [ Rounding; 8 ] =
[
  Rounding::Down,
  Rounding::Up,
  Rounding::HalfEven,
  Rounding::TowardZero,
  Rounding::AwayFromZero,
  Rounding::HalfUp,
  Rounding::HalfDown,
  Rounding::Exact,
];
