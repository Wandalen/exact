//! Zero handling — `minor_zero` and `minor_is_zero`.
//!
//! Both are public and re-exported by the facade, yet nothing called
//! either one, this crate's own suite included.

use exact_minor::{ Backing, minor_from_i64, minor_is_zero, minor_to_i64, minor_zero };

/// `minor_zero` is exactly the integer zero.
#[ test ]
fn zero_is_the_integer_zero()
{
  assert_eq!( minor_to_i64( minor_zero() ), 0 );
}

/// `minor_is_zero` is true for zero and for nothing else, both extremes included.
#[ test ]
fn only_zero_is_zero()
{
  assert!( minor_is_zero( minor_zero() ) );
  for value in [ 1, -1, Backing::MAX, Backing::MIN ]
  {
    assert!( !minor_is_zero( minor_from_i64( value ) ), "{value} was reported as zero" );
  }
}
