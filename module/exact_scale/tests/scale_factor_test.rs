//! The power-of-ten table and the declared ceiling's own headroom relation.

use exact_scale::{ CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS, HEADROOM_FACTOR, MONEY_SCALE, pow10 };

/// `pow10` matches the ordinary powers of ten at both the identity and a real scale.
#[ test ]
fn pow10_matches_ordinary_powers_of_ten()
{
  assert_eq!( pow10( 0 ), 1 );
  assert_eq!( pow10( 1 ), 10 );
  assert_eq!( pow10( 6 ), 1_000_000 );
}

/// The declared ceiling stays a thousandfold below the backing width.
///
/// The headroom relation is a compile-time fact — the crate asserts it where
/// the constant is declared — and this test pins down the part the
/// inequality alone does not: that `CEILING_MINOR_UNITS` really is the
/// declared whole-unit ceiling expressed at the money scale.
#[ test ]
fn the_ceiling_stays_a_thousandfold_below_the_backing_width()
{
  assert_eq!( CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS * pow10( MONEY_SCALE ) );
  const { assert!( CEILING_MINOR_UNITS <= i64::MAX / HEADROOM_FACTOR ) };
  assert!
  (
    CEILING_MINOR_UNITS.checked_mul( HEADROOM_FACTOR ).is_some(),
    "a thousand ceiling-sized amounts must be summable without leaving the width",
  );
}

/// `pow10` panics past `n = 18`, the largest power of ten an `i64` holds.
#[ test ]
#[ should_panic( expected = "10^n exceeds the backing width" ) ]
fn pow10_panics_past_the_backing_widths_largest_power()
{
  let _ = pow10( 19 );
}

/// `pow10` is exact for every scale the backing width can hold, the top one included.
#[ test ]
fn pow10_is_exact_up_to_the_widest_power()
{
  for n in 0..=18
  {
    assert_eq!( pow10( n ), 10_i64.pow( n ), "10^{n}" );
  }
}
