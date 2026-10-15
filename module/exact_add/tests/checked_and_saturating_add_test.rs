//! Checked arithmetic dispatched per kind, and saturating arithmetic
//! clamping to the declared ceiling instead of refusing.

use exact_add::
{
  money_add, money_checked_neg, money_saturating_add, money_sub, price_add, price_sub, qty_add,
  qty_saturating_add, qty_sub,
};
use exact_kind::{ KindError, Money, Price, Quantity };

/// Checked addition and subtraction dispatch to the same exact result as
/// calling the method on `exact_kind` directly.
#[ test ]
fn checked_add_and_sub_dispatch_to_the_kind_s_own_arithmetic()
{
  let a = Money::parse( "0.1" ).unwrap();
  let b = Money::parse( "0.2" ).unwrap();
  assert_eq!( money_add( a, b ).unwrap(), Money::parse( "0.3" ).unwrap() );
  assert_eq!( money_sub( a.checked_add( b ).unwrap(), b ).unwrap(), a );
}

/// Quantity addition and subtraction carry the non-negativity refusal
/// through, same as calling `Qty`'s own methods directly.
#[ test ]
fn qty_add_and_sub_carry_the_non_negative_refusal_through()
{
  let held = Quantity::from_int( 3 ).unwrap();
  let taken = Quantity::from_int( 5 ).unwrap();

  assert_eq!( qty_add( held, taken ).unwrap().whole(), 8 );
  assert!( matches!( qty_sub( held, taken ), Err( KindError::Negative { .. } ) ) );
}

/// Price arithmetic gives the same exact results as money arithmetic — a
/// `Price` wraps a `Money`, so its own `checked_add`/`checked_sub` do the
/// same integer work.
#[ test ]
fn price_add_and_sub_dispatch_the_same_as_money()
{
  let a = Price::parse( "10.5" ).unwrap();
  let b = Price::parse( "0.5" ).unwrap();
  assert_eq!( price_add( a, b ).unwrap(), Price::parse( "11" ).unwrap() );
  assert_eq!( price_sub( a, b ).unwrap(), Price::parse( "10" ).unwrap() );
}

/// Money negation is always allowed — money is signed.
#[ test ]
fn money_checked_neg_negates_at_both_signs()
{
  let a = Money::parse( "5" ).unwrap();
  assert_eq!( money_checked_neg( a ).unwrap(), Money::parse( "-5" ).unwrap() );
  assert_eq!( money_checked_neg( money_checked_neg( a ).unwrap() ).unwrap(), a );
}

/// Saturating money addition clamps to the declared ceiling at both signs,
/// instead of refusing.
#[ test ]
fn money_saturating_add_clamps_to_the_declared_ceiling_at_both_signs()
{
  let neg_epsilon = Money::EPSILON.checked_neg().unwrap();
  assert_eq!( money_saturating_add( Money::MAX, Money::EPSILON ), Money::MAX );
  assert_eq!( money_saturating_add( Money::MIN, neg_epsilon ), Money::MIN );
}

/// Saturating money addition matches checked addition everywhere in range.
#[ test ]
fn money_saturating_add_matches_checked_addition_in_range()
{
  let a = Money::parse( "100" ).unwrap();
  let b = Money::parse( "50" ).unwrap();
  assert_eq!( money_saturating_add( a, b ), money_add( a, b ).unwrap() );
}

/// Saturating quantity addition clamps to the ceiling, never below zero.
#[ test ]
fn qty_saturating_add_clamps_to_the_declared_ceiling()
{
  assert_eq!( qty_saturating_add( Quantity::MAX, Quantity::EPSILON ), Quantity::MAX );

  let a = Quantity::from_int( 3 ).unwrap();
  let b = Quantity::from_int( 4 ).unwrap();
  assert_eq!( qty_saturating_add( a, b ), qty_add( a, b ).unwrap() );
}

/// Every checked addition refuses a sum past the declared ceiling — money,
/// price and quantity alike — naming the offending minor count.
#[ test ]
fn checked_add_refuses_a_sum_past_the_ceiling_for_every_kind()
{
  let one = Money::from_minor( 1 ).unwrap();
  let past = Money::MAX.minor() + 1;
  assert_eq!( money_add( Money::MAX, one ), Err( KindError::ExceedsCeiling { minor : past } ) );
  let refused = KindError::ExceedsCeiling { minor : past };
  assert_eq!( price_add( Price::MAX, Price::from_minor( 1 ).unwrap() ), Err( refused ) );
  assert_eq!( qty_add( Quantity::MAX, Quantity::EPSILON ), Err( refused ) );
}

/// Checked subtraction refuses a difference below the negative ceiling, for
/// the two signed kinds.
#[ test ]
fn checked_sub_refuses_a_difference_below_the_negative_ceiling()
{
  let one = Money::from_minor( 1 ).unwrap();
  let below = Money::MIN.minor() - 1;
  assert_eq!( money_sub( Money::MIN, one ), Err( KindError::ExceedsCeiling { minor : below } ) );
  let lowest_price = Price::from_minor( Money::MIN.minor() ).unwrap();
  let one_price = Price::from_minor( 1 ).unwrap();
  assert_eq!( price_sub( lowest_price, one_price ), Err( KindError::ExceedsCeiling { minor : below } ) );
}

/// A quantity may be drawn down to exactly zero — the refusal starts one
/// minor unit below it, and names that unit.
#[ test ]
fn qty_sub_reaches_exactly_zero_and_refuses_one_unit_below()
{
  let five = Quantity::from_int( 5 ).unwrap();
  assert_eq!( qty_sub( five, five ).unwrap(), Quantity::ZERO );
  assert_eq!( qty_sub( Quantity::ZERO, Quantity::EPSILON ), Err( KindError::Negative { minor : -1 } ) );
}

/// Prices may be negative, and add and subtract across zero exactly.
#[ test ]
fn price_add_and_sub_cross_zero_exactly()
{
  let minus_one = Price::parse( "-1" ).unwrap();
  let half = Price::parse( "0.5" ).unwrap();
  assert_eq!( price_add( minus_one, half ).unwrap(), Price::parse( "-0.5" ).unwrap() );
  assert_eq!( price_sub( half, Price::parse( "1" ).unwrap() ).unwrap(), Price::parse( "-0.5" ).unwrap() );
}

/// Negating zero gives zero, and negating either end of the symmetric range
/// gives the other end.
#[ test ]
fn money_checked_neg_maps_zero_to_zero_and_each_end_to_the_other()
{
  assert_eq!( money_checked_neg( Money::ZERO ).unwrap(), Money::ZERO );
  assert_eq!( money_checked_neg( Money::MAX ).unwrap(), Money::MIN );
  assert_eq!( money_checked_neg( Money::MIN ).unwrap(), Money::MAX );
}
