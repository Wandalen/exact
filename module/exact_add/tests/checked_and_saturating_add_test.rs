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

/// Price arithmetic dispatches the same as money arithmetic — same
/// underlying type today, per the disclosed `Price == Money` deviation.
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
