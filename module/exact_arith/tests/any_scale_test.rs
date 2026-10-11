//! Every operation works at any scale, not only the standard money scale:
//! a price in cents (2 places) and a quantity in satoshis (8 places).

use exact_arith::
{
  Decimal, DustTo, PriceAt, Qty, RatioError, Rounding, SnapError, TickAt, WireError,
  money_from_wire, money_sum_assert_zero, money_to_wire, price_add, price_cmp, price_fmt,
  price_from_str, price_mul_qty, price_snap_tick, qty_dust_split,
};

fn cents( text : &str ) -> PriceAt< 2 >
{
  PriceAt::parse( text ).unwrap()
}

/// A price in cents adds, compares and renders at its own scale.
#[ test ]
fn a_price_in_cents_adds_compares_and_renders()
{
  let sum = price_add( cents( "65000.25" ), cents( "0.75" ) ).unwrap();
  assert_eq!( price_fmt( sum ), "65001" );
  assert_eq!( price_cmp( cents( "1.5" ), cents( "2" ) ), core::cmp::Ordering::Less );
  assert_eq!( price_from_str::< 2 >( "1.5" ).unwrap(), cents( "1.5" ) );
}

/// A price in cents times a quantity in satoshis costs money in cents.
#[ test ]
fn a_price_in_cents_times_a_quantity_in_satoshis_costs_money_in_cents()
{
  let btc : Qty< 8 > = Qty::parse( "0.12345678" ).unwrap();
  let cost = price_mul_qty( cents( "65000" ), btc, Rounding::HalfEven ).unwrap();
  assert_eq!( cost, Decimal::< 2 >::parse( "8024.69" ).unwrap() );
  assert_eq!( price_mul_qty( cents( "65000" ), btc, Rounding::Exact ), Err( RatioError::Inexact ) );
}

/// A price in cents snaps to a tick in cents.
#[ test ]
fn a_price_in_cents_snaps_to_a_tick_in_cents()
{
  let tick = TickAt::new( cents( "0.05" ) ).unwrap();
  assert_eq!( price_snap_tick( cents( "1.27" ), tick, Rounding::Down ).unwrap(), cents( "1.25" ) );
  assert_eq!( price_snap_tick( cents( "1.27" ), tick, Rounding::Exact ), Err( SnapError::OffGrid ) );
}

/// A quantity in satoshis splits into satoshis, the dust in the first part.
#[ test ]
fn a_quantity_in_satoshis_splits_into_satoshis()
{
  let total : Qty< 8 > = Qty::from_minor( 10 ).unwrap();
  let parts = qty_dust_split( total, 3, Rounding::Down, DustTo::First ).unwrap();
  assert_eq!( parts.iter().map( | q | q.minor() ).collect::< Vec< _ > >(), [ 4, 3, 3 ] );
}

/// Money in cents sums to zero at its own scale.
#[ test ]
fn money_in_cents_sums_to_zero()
{
  let paid : Decimal< 2 > = Decimal::parse( "-10.50" ).unwrap();
  let received : Decimal< 2 > = Decimal::parse( "10.50" ).unwrap();
  assert_eq!( money_sum_assert_zero( &[ paid, received ] ), Ok( () ) );
}

/// The wire records the scale, so money in cents decodes as cents and is
/// refused at any other scale — never read as millionths.
#[ test ]
fn the_wire_keeps_the_scale_and_refuses_another()
{
  let v : Decimal< 2 > = Decimal::parse( "12.34" ).unwrap();
  let wire = money_to_wire( v );
  assert_eq!( wire.scale(), 2 );
  assert_eq!( money_from_wire( wire ), Ok( v ) );
  assert_eq!( money_from_wire::< 6 >( wire ), Err( WireError::BadScale ) );
}
