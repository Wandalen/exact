//! Comparison, equality, and min/max dispatch to the kind's own derived
//! `Ord`/`PartialEq` exactly.

use core::cmp::Ordering;
use exact_cmp::{ money_cmp, money_eq, price_cmp, price_max, price_min, qty_cmp };
use exact_kind::{ Money, Price, Quantity };

/// Ordering dispatches to the same result as the kind's own `Ord`.
#[ test ]
fn cmp_dispatches_to_the_kind_s_own_ord()
{
  let a = Money::parse( "1" ).unwrap();
  let b = Money::parse( "2" ).unwrap();
  assert_eq!( money_cmp( a, b ), Ordering::Less );
  assert_eq!( money_cmp( b, a ), Ordering::Greater );
  assert_eq!( money_cmp( a, a ), Ordering::Equal );
}

/// Quantity and price ordering dispatch the same way as money.
#[ test ]
fn qty_and_price_ordering_dispatch_the_same_way_as_money()
{
  let small = Quantity::from_int( 1 ).unwrap();
  let big = Quantity::from_int( 2 ).unwrap();
  assert_eq!( qty_cmp( small, big ), Ordering::Less );

  let cheap = Price::parse( "1" ).unwrap();
  let dear = Price::parse( "2" ).unwrap();
  assert_eq!( price_cmp( cheap, dear ), Ordering::Less );
}

/// Equality dispatches to the kind's own derived `PartialEq`.
#[ test ]
fn money_eq_dispatches_to_the_kind_s_own_partial_eq()
{
  let a = Money::parse( "1.5" ).unwrap();
  let b = Money::parse( "1.5" ).unwrap();
  let c = Money::parse( "1.6" ).unwrap();
  assert!( money_eq( a, b ) );
  assert!( !money_eq( a, c ) );
}

/// `price_min`/`price_max` pick the lesser/greater value, at both orderings.
#[ test ]
fn price_min_and_max_pick_the_lesser_and_greater_value()
{
  let low = Price::parse( "1" ).unwrap();
  let high = Price::parse( "2" ).unwrap();
  assert_eq!( price_min( low, high ), low );
  assert_eq!( price_min( high, low ), low );
  assert_eq!( price_max( low, high ), high );
  assert_eq!( price_max( high, low ), high );
}
