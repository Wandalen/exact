//! `PriceAt< SCALE >`: a price at its own scale. `Price` is the one at the
//! standard money scale, so it behaves exactly as before.

use exact_kind::{ Decimal, KindError, Price, PriceAt };
use exact_round::Rounding;

/// `Price` is `PriceAt` at six places — one minor unit is a millionth.
#[ test ]
fn price_is_price_at_the_money_scale()
{
  let price : PriceAt< 6 > = Price::parse( "1.5" ).unwrap();
  assert_eq!( price.minor(), 1_500_000 );
}

/// A price in cents: two places, so one minor unit is a cent.
#[ test ]
fn a_price_in_cents_parses_adds_and_renders()
{
  let a : PriceAt< 2 > = PriceAt::parse( "65000.25" ).unwrap();
  let b : PriceAt< 2 > = PriceAt::parse( "0.75" ).unwrap();
  assert_eq!( a.minor(), 6_500_025 );
  assert_eq!( a.checked_add( b ).unwrap().to_string(), "65001" );
}

/// A price in cents refuses a third decimal place, naming its own scale.
#[ test ]
fn a_price_in_cents_refuses_a_third_place()
{
  let got = PriceAt::< 2 >::parse( "1.005" );
  assert_eq!( got, Err( KindError::ExcessPrecision { digits : 3, scale : 2 } ) );
}

/// The ceiling is in minor units, the same at every scale.
#[ test ]
fn the_ceiling_is_the_same_count_of_minor_units_at_every_scale()
{
  assert_eq!( PriceAt::< 2 >::MAX.minor(), Price::MAX.minor() );
  assert_eq!( PriceAt::< 2 >::MAX.minor(), Decimal::< 2 >::MAX.minor() );
}

/// A price rescales as a decimal does: finer is exact, coarser rounds per the
/// mode, and `Exact` refuses a dropped digit.
#[ test ]
fn a_price_rescales_like_a_decimal()
{
  let cents : PriceAt< 2 > = PriceAt::parse( "12.34" ).unwrap();
  assert_eq!( cents.rescale::< 6 >( Rounding::Exact ).unwrap(), Price::parse( "12.34" ).unwrap() );

  let fine = Price::parse( "12.345678" ).unwrap();
  assert_eq!( fine.rescale::< 2 >( Rounding::HalfEven ).unwrap(), PriceAt::< 2 >::parse( "12.35" ).unwrap() );
  assert_eq!( fine.rescale::< 2 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 6, scale : 2 } ) );
  let short = Price::parse( "1.005" ).unwrap();
  assert_eq!( short.rescale::< 2 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 3, scale : 2 } ), "the price's own digits, not its scale" );
}
