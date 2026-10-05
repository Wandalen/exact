//! `Price` as its own kind: a `Money` inside, so the same symmetric range,
//! exact arithmetic and text form — but a distinct type, so a price and an
//! amount of money never mix (that half is the compile_fail doctests on
//! `Price` itself).

use exact_kind::{ KindError, Money, Price };

/// The constants sit where `Money`'s do — `Price` adds no range of its own.
#[ test ]
fn price_constants_match_the_money_range()
{
  assert_eq!( Price::ZERO.minor(), 0 );
  assert_eq!( Price::MAX.minor(), Money::MAX.minor() );
}

/// `from_minor` accepts the whole symmetric range, negatives included, and
/// refuses one unit past either end, naming the offending count.
#[ test ]
fn from_minor_accepts_the_symmetric_range_and_refuses_one_unit_past_it()
{
  let max = Money::MAX.minor();
  assert_eq!( Price::from_minor( max ).unwrap(), Price::MAX );
  assert_eq!( Price::from_minor( -max ).unwrap().minor(), -max );
  assert_eq!( Price::from_minor( max + 1 ), Err( KindError::ExceedsCeiling { minor : max + 1 } ) );
  assert_eq!( Price::from_minor( -max - 1 ), Err( KindError::ExceedsCeiling { minor : -max - 1 } ) );
}

/// Price arithmetic is exact — the classic `0.1 + 0.2` lands on `0.3` — and
/// subtracting what was added returns the original.
#[ test ]
fn price_arithmetic_is_exact()
{
  let a = Price::parse( "0.1" ).unwrap();
  let b = Price::parse( "0.2" ).unwrap();
  let sum = a.checked_add( b ).unwrap();
  assert_eq!( sum, Price::parse( "0.3" ).unwrap() );
  assert_eq!( sum.checked_sub( b ).unwrap(), a );
}

/// Price arithmetic refuses a result past the ceiling at both ends.
#[ test ]
fn price_arithmetic_refuses_a_result_past_the_ceiling_at_both_ends()
{
  let one = Price::from_minor( 1 ).unwrap();
  let lowest = Price::from_minor( -Money::MAX.minor() ).unwrap();
  assert!( matches!( Price::MAX.checked_add( one ), Err( KindError::ExceedsCeiling { .. } ) ) );
  assert!( matches!( lowest.checked_sub( one ), Err( KindError::ExceedsCeiling { .. } ) ) );
}

/// A price parses and renders exactly like money: trailing zeros trimmed, a
/// negative sign kept, zero as plain `0`.
#[ test ]
fn price_parses_and_renders_like_money()
{
  for ( text, shown ) in [ ( "1.25", "1.25" ), ( "1.50", "1.5" ), ( "-0.05", "-0.05" ), ( "0", "0" ) ]
  {
    assert_eq!( Price::parse( text ).unwrap().to_string(), shown );
  }
}

/// A price refuses the same malformed and over-precise text money does.
#[ test ]
fn price_parse_refuses_what_money_parse_refuses()
{
  let too_precise = Price::parse( "1.2345678" );
  assert!( matches!( too_precise, Err( KindError::ExcessPrecision { digits : 7, scale : 6 } ) ) );
  for bad in [ "", "abc", "1.", ".5", "NaN" ]
  {
    let got = Price::parse( bad );
    assert!( matches!( got, Err( KindError::Malformed { .. } ) ), "{bad:?} should be malformed" );
  }
}

/// Prices order by value, negatives below zero — the ordering `exact_cmp`'s
/// `price_cmp`/`price_min`/`price_max` rely on.
#[ test ]
fn prices_order_by_value_with_negatives_below_zero()
{
  let below = Price::from_minor( -1 ).unwrap();
  let above = Price::from_minor( 1 ).unwrap();
  assert!( below < Price::ZERO );
  assert!( Price::ZERO < above );
  assert_eq!( below.max( above ), above );
}
