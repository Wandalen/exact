//! `+` and `-` on `Decimal`, `Qty` and `PriceAt` are each kind's
//! `checked_add` and `checked_sub` spelled as operators: they return a
//! `Result`, and a chain stops at the first error.

use exact_kind::{ Decimal, KindError, Money };

fn cents( text : &str ) -> Decimal< 2 >
{
  Decimal::parse( text ).unwrap()
}

/// `a + b` is exactly `a.checked_add( b )`, and `a - b` is `a.checked_sub( b )`.
#[ test ]
fn operators_are_the_checked_methods()
{
  let ( a, b ) = ( cents( "1.50" ), cents( "2.25" ) );
  assert_eq!( a + b, a.checked_add( b ) );
  assert_eq!( a - b, a.checked_sub( b ) );
  assert_eq!( ( a + b ).unwrap(), cents( "3.75" ) );
  assert_eq!( ( a - b ).unwrap(), cents( "-0.75" ) );
}

/// A chain needs one `?` at the end and carries the first error to it.
#[ test ]
fn a_chain_stops_at_the_first_error()
{
  let ( a, b, c ) = ( cents( "1.00" ), cents( "2.00" ), cents( "0.50" ) );
  assert_eq!( ( a + b + c ).unwrap(), cents( "3.50" ) );
  assert_eq!( ( a + b - c ).unwrap(), cents( "2.50" ) );
  let past = Money::MAX + Money::EPSILON + Money::EPSILON;
  assert!( matches!( past, Err( KindError::ExceedsCeiling { .. } ) ), "{past:?}" );
  assert_eq!( Money::MAX + Money::EPSILON - Money::EPSILON, Money::MAX + Money::EPSILON, "the first error wins" );
}

/// A quantity below zero is refused, not wrapped; a price adds like money.
#[ test ]
fn each_kind_keeps_its_own_rules()
{
  use exact_kind::{ PriceAt, Qty };
  let ( one, two ) = ( Qty::< 8 >::parse( "1" ).unwrap(), Qty::< 8 >::parse( "2" ).unwrap() );
  assert!( matches!( one - two, Err( KindError::Negative { .. } ) ) );
  assert_eq!( ( two - one ).unwrap(), one );
  let p = PriceAt::< 2 >::parse( "65000.00" ).unwrap();
  assert_eq!( ( p + p ).unwrap(), PriceAt::< 2 >::parse( "130000.00" ).unwrap() );
}

/// `Qty` and `PriceAt` add, subtract and chain exactly as `Decimal` does —
/// each of their impls, on a value and on a `Result`, is that kind's own
/// checked method, and a chain carries the first error to its end.
#[ test ]
fn every_kind_adds_subtracts_and_chains()
{
  use exact_kind::{ PriceAt, Qty, Quantity };
  let q = | text : &str | Qty::< 8 >::parse( text ).unwrap();
  assert_eq!( q( "1" ) + q( "2" ), q( "1" ).checked_add( q( "2" ) ) );
  assert_eq!( ( q( "1" ) + q( "2" ) + q( "0.5" ) ).unwrap(), q( "3.5" ) );
  assert_eq!( ( q( "3" ) - q( "1" ) - q( "0.5" ) ).unwrap(), q( "1.5" ) );
  assert_eq!( ( q( "1" ) + q( "2" ) - q( "0.5" ) ).unwrap(), q( "2.5" ) );
  assert!( matches!( q( "1" ) - q( "2" ) + q( "5" ), Err( KindError::Negative { .. } ) ), "the first error wins" );
  assert!( matches!( Quantity::MAX + Quantity::EPSILON, Err( KindError::ExceedsCeiling { .. } ) ) );
  let p = | text : &str | PriceAt::< 2 >::parse( text ).unwrap();
  assert_eq!( p( "5.00" ) - p( "1.25" ), p( "5.00" ).checked_sub( p( "1.25" ) ) );
  assert_eq!( ( p( "1.00" ) + p( "2.00" ) + p( "0.50" ) ).unwrap(), p( "3.50" ) );
  assert_eq!( ( p( "5.00" ) - p( "1.00" ) - p( "0.50" ) ).unwrap(), p( "3.50" ) );
  assert_eq!( ( p( "1.00" ) + p( "2.00" ) - p( "0.50" ) ).unwrap(), p( "2.50" ) );
}
