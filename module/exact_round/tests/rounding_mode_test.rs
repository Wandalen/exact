//! Rounding mode classification and the family's default policy.

use exact_round::{ Rounding, rounding_default, rounding_name };

/// The family's default is `Down`, as the design specifies.
#[ test ]
fn the_default_rounding_mode_is_down()
{
  assert_eq!( rounding_default(), Rounding::Down );
}

/// Every rounding mode has a stable, lowercase, snake_case name.
#[ test ]
fn every_rounding_mode_has_a_stable_name()
{
  assert_eq!( rounding_name( Rounding::Down ), "down" );
  assert_eq!( rounding_name( Rounding::Up ), "up" );
  assert_eq!( rounding_name( Rounding::HalfEven ), "half_even" );
}

/// `Rounding` is a plain, comparable, copyable enum — a policy value, not a
/// resource.
#[ test ]
fn rounding_is_copy_and_comparable()
{
  let a = Rounding::Down;
  let b = a;
  assert_eq!( a, b );
  assert_ne!( Rounding::Down, Rounding::Up );
}
