//! Test Matrix T05, T06 — a quantity that refuses to go below zero.
//!
//! The type's whole reason for existing is one refusal, so the tests are mostly
//! about where that refusal sits. Zero must remain reachable — a hold emptied
//! exactly is not an error — and one minor unit past it must not.

use exact_kind::{ Decimal, KindError, Quantity, Qty };
use exact_minor::Backing;
use exact_scale::CEILING_WHOLE_UNITS;

/// T05 — withdrawing more than is held is an error, not an underflow.
///
/// The error carries the count the operation would have produced, so the report
/// says how short the hold was rather than only that it was short.
#[ test ]
fn withdrawing_more_than_is_held_is_refused()
{
  let held = Quantity::from_int( 3 ).unwrap();
  let taken = Quantity::from_int( 5 ).unwrap();

  assert_eq!( held.checked_sub( taken ), Err( KindError::Negative { minor : -2_000_000 } ) );
}

/// T05 — the refusal starts exactly one minor unit below zero.
///
/// Emptying a hold to exactly zero succeeds; the next smallest step does not.
/// A clamp-to-zero implementation would pass the first assertion and fail the
/// second, which is the point of taking them together.
#[ test ]
fn zero_is_reachable_and_one_unit_below_it_is_not()
{
  let held = Quantity::from_int( 1 ).unwrap();

  assert_eq!( held.checked_sub( held ).unwrap(), Quantity::ZERO );
  assert!( matches!( Quantity::ZERO.checked_sub( Quantity::EPSILON ), Err( KindError::Negative { .. } ) ) );
}

/// T06 — an integer survives the round trip through the quantity type.
///
/// Taken at both ends of the accepted range as well as in the middle, so a
/// scaling bug that only shows up near the ceiling is not hidden by small
/// friendly values.
#[ test ]
fn an_integer_quantity_round_trips_to_the_same_integer()
{
  for whole in [ 0, 1, 7, 1_000_000, CEILING_WHOLE_UNITS ]
  {
    let qty = Quantity::from_int( whole ).unwrap();
    assert_eq!( qty.whole(), whole );
    assert_eq!( qty.to_string(), whole.to_string() );
  }
}

/// A negative constructor argument is refused at every entry point.
///
/// Four ways in, one invariant. A type whose invariant holds for three of its
/// four constructors does not have an invariant.
#[ test ]
fn every_constructor_refuses_a_negative_value()
{
  assert!( matches!( Quantity::from_int( -1 ), Err( KindError::Negative { .. } ) ) );
  assert!( matches!( Quantity::from_minor( -1 ), Err( KindError::Negative { .. } ) ) );
  assert!( matches!( Quantity::parse( "-0.000001" ), Err( KindError::Negative { .. } ) ) );
  assert!
  (
    matches!
    (
      Quantity::from_decimal( Decimal::parse( "-1" ).unwrap() ),
      Err( KindError::Negative { .. } ),
    ),
  );
  // Negative zero is not negative: `-0` denotes zero, which a quantity holds.
  assert_eq!( Quantity::parse( "-0.0" ).unwrap(), Quantity::ZERO );
}

/// Range failures stay distinguishable from the non-negativity failure.
///
/// Both are errors, and treating them as one error would send an investigation
/// into the range budget when the actual event was a withdrawal that asked for
/// more than was there.
#[ test ]
fn a_ceiling_breach_reports_as_a_range_error_and_not_as_a_negative()
{
  let breach = Quantity::from_int( CEILING_WHOLE_UNITS + 1 );
  assert!( matches!( breach, Err( KindError::ExceedsCeiling { .. } ) ) );

  let ceiling = Quantity::from_int( CEILING_WHOLE_UNITS ).unwrap();
  assert!( matches!( ceiling.checked_add( Quantity::EPSILON ), Err( KindError::ExceedsCeiling { .. } ) ) );
}

/// Addition and scalar multiplication carry the invariant through.
#[ test ]
fn the_arithmetic_that_stays_in_range_stays_non_negative()
{
  let two = Quantity::from_int( 2 ).unwrap();

  assert_eq!( two.checked_add( two ).unwrap().whole(), 4 );
  assert_eq!( two.checked_mul_int( 3 ).unwrap().whole(), 6 );
  assert_eq!( two.checked_mul_int( 0 ).unwrap(), Quantity::ZERO );
  assert!( matches!( two.checked_mul_int( -1 ), Err( KindError::Negative { .. } ) ) );
}

/// A ceiling breach reached through multiplication reports as a range error, not a negative.
///
/// The test above pins this distinction only through `checked_add`;
/// `checked_mul_int` forwards its own range-error arm and nothing exercised
/// it — every multiplication test either stays in range or hits the
/// negative-multiplier guard, which is a different branch entirely.
#[ test ]
fn checked_mul_int_ceiling_breach_reports_as_a_range_error_and_not_as_a_negative()
{
  let near_ceiling = Quantity::from_int( CEILING_WHOLE_UNITS ).unwrap();
  assert!( matches!( near_ceiling.checked_mul_int( 2 ), Err( KindError::ExceedsCeiling { .. } ) ) );
}

/// A zero quantity times a negative multiplier is still zero, not a refusal.
///
/// Nothing above exercises the exact claim `checked_mul_int`'s own doc comment
/// makes — that a zero quantity times *any* multiplier, negative included, is
/// still zero, because `n < 0` alone does not guarantee the error. An
/// implementation that shortcut to `Negative` whenever `n < 0`, without
/// checking whether `self` is already zero, would pass every other test in
/// this file and only fail here.
#[ test ]
fn a_zero_quantity_times_a_negative_multiplier_is_still_zero()
{
  assert_eq!( Quantity::ZERO.checked_mul_int( -1 ).unwrap(), Quantity::ZERO );
  assert_eq!( Quantity::ZERO.checked_mul_int( Backing::MIN ).unwrap(), Quantity::ZERO );
}

/// `MAX` sits exactly on the declared ceiling, same as the underlying decimal.
#[ test ]
fn max_sits_exactly_on_the_declared_ceiling()
{
  assert_eq!( Quantity::MAX.whole(), CEILING_WHOLE_UNITS );
  assert!( Quantity::MAX.checked_add( Quantity::EPSILON ).is_err() );
}

/// The decimal beneath is reachable, and getting back in goes through the check.
///
/// `as_decimal` is the escape hatch for arithmetic that legitimately leaves the
/// type — a price times a quantity, most obviously. It hands back a signed
/// value on purpose: the only way to a `Qty` again is `from_decimal`, which is
/// where the refusal lives.
#[ test ]
fn leaving_the_type_and_coming_back_passes_through_the_refusal()
{
  let qty : Qty< 6 > = Quantity::parse( "1.5" ).unwrap();
  let signed = qty.as_decimal();

  assert_eq!( signed.minor(), 1_500_000 );
  assert_eq!( Quantity::from_decimal( signed ).unwrap(), qty );
  assert!( Quantity::from_decimal( signed.checked_neg().unwrap() ).is_err() );
}
