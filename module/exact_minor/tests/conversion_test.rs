//! `Minor` conversions — `minor_from_i64` and `minor_to_i64`, the only way
//! a raw `i64` becomes a count of minor units and back.

use exact_minor::{ Backing, minor_from_i64, minor_to_i64 };

/// Wrapping and unwrapping returns the same number, both extremes included.
#[ test ]
fn from_and_to_i64_round_trip()
{
  for value in [ 0, 1, -1, Backing::MAX, Backing::MIN ]
  {
    assert_eq!( minor_to_i64( minor_from_i64( value ) ), value );
  }
}

/// `Minor` orders exactly like the number it holds.
#[ test ]
fn minor_orders_like_the_number_it_holds()
{
  assert!( minor_from_i64( -1 ) < minor_from_i64( 0 ) );
  assert!( minor_from_i64( Backing::MAX ) > minor_from_i64( Backing::MIN ) );
}
