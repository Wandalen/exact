//! Saturating arithmetic — new in this crate, with no real-code precedent:
//! the family's prior shape only ever offered checked operations. These
//! clamp instead of refusing, for call sites that have already decided a
//! clamped answer to range failure is acceptable.

use exact_minor::{ Backing, minor_saturating_add, minor_saturating_sub };

/// In-range saturating arithmetic matches checked arithmetic exactly.
#[ test ]
fn in_range_saturating_arithmetic_matches_the_exact_sum()
{
  assert_eq!( minor_saturating_add( 300_000, 200_000 ), 500_000 );
  assert_eq!( minor_saturating_sub( 500_000, 200_000 ), 300_000 );
}

/// Addition past the backing maximum clamps to the maximum rather than wrapping.
#[ test ]
fn addition_clamps_to_the_backing_maximum()
{
  assert_eq!( minor_saturating_add( Backing::MAX, 1 ), Backing::MAX );
  assert_eq!( minor_saturating_add( Backing::MAX, Backing::MAX ), Backing::MAX );
}

/// Subtraction past the backing minimum clamps to the minimum rather than wrapping.
#[ test ]
fn subtraction_clamps_to_the_backing_minimum()
{
  assert_eq!( minor_saturating_sub( Backing::MIN, 1 ), Backing::MIN );
  assert_eq!( minor_saturating_sub( Backing::MIN, Backing::MAX ), Backing::MIN );
}

/// Clamping goes to the bound the result actually crossed, in both directions.
#[ test ]
fn clamping_follows_the_direction_of_overflow()
{
  assert_eq!( minor_saturating_add( Backing::MIN, -1 ), Backing::MIN );
  assert_eq!( minor_saturating_sub( Backing::MAX, -1 ), Backing::MAX );
}
