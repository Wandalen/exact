//! Test Matrix T02, T03, T04 — exactness, and refusal at the edge.
//!
//! Two claims are under test and they pull in opposite directions. The first is
//! that ordinary arithmetic is exact: `0.1 + 0.2` is `0.3` and not something
//! near it. The second is that arithmetic which cannot be exact is *refused*:
//! no wrap, no saturation, no panic, just an error naming which operation ran
//! out of room.
//!
//! Every range case below is taken at the boundary rather than at some large
//! round number. A check that returned `Ok` for everything up to a generous
//! margin would pass a test built from comfortable values, and the comfortable
//! value is not where an overflow actually happens.

use exact_kind::{ Decimal, KindError, Money };
use exact_minor::Backing;
use exact_scale::{ CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS, HEADROOM_FACTOR };

/// T02 — `0.1 + 0.2 == 0.3`, the case binary floating point cannot represent.
///
/// The lane's control arm exists to show the same sum failing in `f64`; this
/// test is the exact half of that pair, asserted on the value rather than on
/// its rendering so a lenient printer cannot supply the agreement.
#[ test ]
fn a_tenth_plus_a_fifth_is_exactly_three_tenths()
{
  let a = Money::parse( "0.1" ).unwrap();
  let b = Money::parse( "0.2" ).unwrap();
  let sum = a.checked_add( b ).unwrap();

  assert_eq!( sum, Money::parse( "0.3" ).unwrap() );
  assert_eq!( sum.minor(), 300_000 );
}

/// Repeated addition stays exact — the accumulating form of the same claim.
///
/// One addition being exact is weaker than it looks: the failure this family
/// exists to prevent is a residue that only shows up after it has been added to
/// itself a few thousand times.
#[ test ]
fn ten_thousand_tenths_are_exactly_one_thousand()
{
  let tenth = Money::parse( "0.1" ).unwrap();
  let mut total = Money::ZERO;
  for _ in 0..10_000
  {
    total = total.checked_add( tenth ).unwrap();
  }
  assert_eq!( total, Money::parse( "1000" ).unwrap() );
}

/// Subtraction is the exact inverse of addition, at both signs.
#[ test ]
fn subtracting_what_was_added_returns_the_original()
{
  let a = Money::parse( "123.456789" ).unwrap();
  let b = Money::parse( "-0.999999" ).unwrap();
  assert_eq!( a.checked_add( b ).unwrap().checked_sub( b ).unwrap(), a );
  assert_eq!( a.checked_sub( a ).unwrap(), Money::ZERO );
}

/// T03 — a sum one minor unit past the ceiling is refused; the ceiling itself is not.
///
/// Both halves matter. Refusing the over-large sum shows the check exists;
/// accepting the exact ceiling shows it is placed where it was declared rather
/// than somewhere convenient just inside.
#[ test ]
fn addition_is_refused_exactly_one_unit_past_the_ceiling()
{
  let ceiling = Money::from_minor( CEILING_MINOR_UNITS ).unwrap();

  assert_eq!( ceiling.checked_add( Money::ZERO ).unwrap(), ceiling );
  assert_eq!
  (
    ceiling.checked_add( Money::EPSILON ),
    Err( KindError::ExceedsCeiling { minor : CEILING_MINOR_UNITS + 1 } ),
  );
}

/// T03 — the negative edge is refused symmetrically.
///
/// An unsigned-shaped check written against a signed type passes every positive
/// case and lets the whole negative range through.
#[ test ]
fn subtraction_is_refused_one_unit_past_the_negative_ceiling()
{
  let floor = Money::from_minor( -CEILING_MINOR_UNITS ).unwrap();

  assert!( floor.checked_sub( Money::ZERO ).is_ok() );
  assert_eq!
  (
    floor.checked_sub( Money::EPSILON ),
    Err( KindError::ExceedsCeiling { minor : -CEILING_MINOR_UNITS - 1 } ),
  );
}

/// T04 — a product past the ceiling is refused, at the smallest multiplier that does it.
///
/// Named `checked_mul_int` rather than `checked_mul`: this is value × scalar,
/// which is closed at the operand's scale. Value × value is a different
/// operation whose result is at scale `S₁ + S₂`, and it is out of this crate's
/// scope; giving them one name would let the wrong one be reached silently.
#[ test ]
fn multiplication_is_refused_at_the_smallest_multiplier_that_overflows_the_ceiling()
{
  let half = Money::from_minor( CEILING_MINOR_UNITS / 2 + 1 ).unwrap();

  assert!( half.checked_mul_int( 1 ).is_ok() );
  assert!( matches!( half.checked_mul_int( 2 ), Err( KindError::ExceedsCeiling { .. } ) ) );
  assert_eq!( Money::EPSILON.checked_mul_int( 0 ).unwrap(), Money::ZERO );
}

/// T04 — the backing width itself is checked, not only the ceiling.
///
/// Two different failures, deliberately two different errors. The ceiling is a
/// deployment's declared limit; the width is what the machine can hold. Both
/// return, and neither wraps or panics — including at `Backing::MIN`, where a
/// negation that looks total is not.
#[ test ]
fn the_backing_width_is_refused_separately_from_the_ceiling()
{
  assert_eq!
  (
    Money::from_int( Backing::MAX ),
    Err( KindError::Overflow { operation : "from_int" } ),
  );
  assert_eq!
  (
    Money::from_minor( 2 ).unwrap().checked_mul_int( Backing::MAX ),
    Err( KindError::Overflow { operation : "mul_int" } ),
  );
  assert_eq!
  (
    Decimal::< 0 >::parse( "99999999999999999999" ),
    Err( KindError::Overflow { operation : "parse" } ),
  );
}

/// Negation is total across the accepted range and never yields a wrapped value.
#[ test ]
fn negation_holds_across_the_whole_accepted_range()
{
  for minor in [ 0, 1, -1, CEILING_MINOR_UNITS, -CEILING_MINOR_UNITS ]
  {
    let value = Money::from_minor( minor ).unwrap();
    assert_eq!( value.checked_neg().unwrap().minor(), -minor );
  }
}

/// `checked_add`, `checked_sub` and `checked_neg` can never actually produce
/// `Overflow` — the ceiling makes it dead code at the widest operands this
/// type can construct.
#[ test ]
fn overflow_is_unreachable_through_add_sub_and_neg_at_the_widest_operands()
{
  let ceiling = Money::from_minor( CEILING_MINOR_UNITS ).unwrap();
  let floor = Money::from_minor( -CEILING_MINOR_UNITS ).unwrap();

  assert_eq!
  (
    ceiling.checked_add( ceiling ),
    Err( KindError::ExceedsCeiling { minor : CEILING_MINOR_UNITS * 2 } ),
  );
  assert_eq!
  (
    floor.checked_sub( ceiling ),
    Err( KindError::ExceedsCeiling { minor : -CEILING_MINOR_UNITS * 2 } ),
  );
  assert!( ceiling.checked_neg().is_ok() );
  assert!( floor.checked_neg().is_ok() );

  assert!( Money::from_minor( Backing::MIN ).is_err() );
  assert!( Money::from_minor( Backing::MAX ).is_err() );
}

/// The declared ceiling keeps the headroom the range budget requires.
#[ test ]
fn the_ceiling_stays_a_thousandfold_below_the_backing_width()
{
  assert_eq!( CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS * 1_000_000 );
  const { assert!( CEILING_MINOR_UNITS <= Backing::MAX / HEADROOM_FACTOR ) };
  assert!
  (
    CEILING_MINOR_UNITS.checked_mul( HEADROOM_FACTOR ).is_some(),
    "a thousand ceiling-sized amounts must be summable without leaving the width",
  );
}

/// `from_int` succeeds on an ordinary whole number, at both signs.
#[ test ]
fn from_int_builds_the_expected_minor_count_at_both_signs()
{
  assert_eq!( Money::from_int( 5 ).unwrap().minor(), 5_000_000 );
  assert_eq!( Money::from_int( -5 ).unwrap(), Money::parse( "-5" ).unwrap() );
}

/// `MAX` and `MIN` are exactly the declared ceiling, at both signs.
///
/// The clamp targets saturating arithmetic (in `exact_add`) relies on — one
/// unit past either is refused, and the constants themselves must construct.
#[ test ]
fn max_and_min_sit_exactly_on_the_declared_ceiling()
{
  assert_eq!( Money::MAX.minor(), CEILING_MINOR_UNITS );
  assert_eq!( Money::MIN.minor(), -CEILING_MINOR_UNITS );
  assert_eq!( Money::MAX, Money::from_minor( CEILING_MINOR_UNITS ).unwrap() );
  assert_eq!( Money::MIN, Money::from_minor( -CEILING_MINOR_UNITS ).unwrap() );
  assert!( Money::MAX.checked_add( Money::EPSILON ).is_err() );
  assert!( Money::MIN.checked_sub( Money::EPSILON ).is_err() );
}
