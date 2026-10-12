//! `price_mul_qty_to`: a trade's cost at a scale the caller picks, rounded
//! once. Converting `price_mul_qty`'s result instead rounds twice, and can
//! land a minor unit off (review of #6, P6-1).

use exact_kind::{ Decimal, PriceAt, Qty };
use exact_ratio::{ RatioError, price_mul_qty, price_mul_qty_to };
use exact_round::Rounding;

const ROUNDING_MODES : [ Rounding; 7 ] =
[
  Rounding::Down, Rounding::Up, Rounding::HalfEven, Rounding::TowardZero,
  Rounding::AwayFromZero, Rounding::HalfUp, Rounding::HalfDown,
];

/// The reviewer's case: `0.14999999 × 0.1` is exactly `0.014999999`, whose
/// nearest cent is `0.01`. Rounding to the price's scale first gives `0.015`,
/// a tie, which `HalfEven` then takes to `0.02` — a cent off.
#[ test ]
fn one_rounding_lands_on_the_nearest_cent_where_two_do_not()
{
  let price : PriceAt< 8 > = PriceAt::parse( "0.14999999" ).unwrap();
  let qty : Qty< 1 > = Qty::parse( "0.1" ).unwrap();

  let twice = price_mul_qty( price, qty, Rounding::HalfEven ).unwrap().rescale::< 2 >( Rounding::HalfEven ).unwrap();
  assert_eq!( twice, Decimal::< 2 >::parse( "0.02" ).unwrap(), "two roundings: the defect this function avoids" );

  let once : Decimal< 2 > = price_mul_qty_to( price, qty, Rounding::HalfEven ).unwrap();
  assert_eq!( once, Decimal::< 2 >::parse( "0.01" ).unwrap() );
}

/// The same with small numbers: `0.149 × 0.1 = 0.0149` is `0.01` to the cent.
#[ test ]
fn a_cost_below_half_a_cent_rounds_down_once()
{
  let price : PriceAt< 3 > = PriceAt::parse( "0.149" ).unwrap();
  let qty : Qty< 1 > = Qty::parse( "0.1" ).unwrap();
  let cents : Decimal< 2 > = price_mul_qty_to( price, qty, Rounding::HalfEven ).unwrap();
  assert_eq!( cents.minor(), 1 );
  let negative = PriceAt::< 3 >::parse( "-0.149" ).unwrap();
  let refund : Decimal< 2 > = price_mul_qty_to( negative, qty, Rounding::HalfEven ).unwrap();
  assert_eq!( refund.minor(), -1, "a negative price rounds the same way" );
}

/// At the price's own scale it is exactly `price_mul_qty`, under every mode.
#[ test ]
fn at_the_price_s_scale_it_matches_price_mul_qty()
{
  for mode in ROUNDING_MODES
  {
    for price_minor in [ -1_234_567, -5, -1, 0, 1, 5, 7, 999, 1_234_567 ]
    {
      for qty_minor in [ 0, 1, 3, 5, 7, 49, 50, 51, 150, 12_345 ]
      {
        let price = PriceAt::< 3 >::from_minor( price_minor ).unwrap();
        let qty = Qty::< 2 >::from_minor( qty_minor ).unwrap();
        let direct = price_mul_qty( price, qty, mode );
        let to_p : Result< Decimal< 3 >, _ > = price_mul_qty_to( price, qty, mode );
        assert_eq!( direct, to_p, "{price_minor} × {qty_minor} under {mode:?}" );
      }
    }
  }
}

/// A scale finer than `P + Q` needs no rounding: the exact cost is kept, and
/// `Exact` accepts it.
#[ test ]
fn a_finer_scale_keeps_the_exact_cost()
{
  let price : PriceAt< 2 > = PriceAt::parse( "1.25" ).unwrap();
  let qty : Qty< 1 > = Qty::parse( "0.5" ).unwrap();
  let cost : Decimal< 6 > = price_mul_qty_to( price, qty, Rounding::Exact ).unwrap();
  assert_eq!( cost, Decimal::< 6 >::parse( "0.625" ).unwrap() );
}

/// `Exact` refuses a cost that does not land on scale `M`, and accepts one
/// that does.
#[ test ]
fn exact_refuses_a_cost_off_the_target_scale()
{
  let price : PriceAt< 3 > = PriceAt::parse( "0.149" ).unwrap();
  let qty : Qty< 1 > = Qty::parse( "0.1" ).unwrap();
  let cents : Result< Decimal< 2 >, _ > = price_mul_qty_to( price, qty, Rounding::Exact );
  assert_eq!( cents, Err( RatioError::Inexact ) );
  let fine : Decimal< 4 > = price_mul_qty_to( price, qty, Rounding::Exact ).unwrap();
  assert_eq!( fine.minor(), 149 );
}

/// The ceiling counts minor units, so it holds at scale `M`: twice the
/// largest price fits in cents, but not at six places, nor widened to 18.
#[ test ]
fn the_ceiling_is_checked_at_the_target_scale()
{
  let price = PriceAt::< 6 >::MAX;
  let qty : Qty< 0 > = Qty::from_minor( 2 ).unwrap();
  let cents : Decimal< 2 > = price_mul_qty_to( price, qty, Rounding::Down ).unwrap();
  assert_eq!( cents, Decimal::< 2 >::parse( "18000000000" ).unwrap() );
  let six : Result< Decimal< 6 >, _ > = price_mul_qty_to( price, qty, Rounding::Down );
  assert_eq!( six, Err( RatioError::Overflow ) );
  let wide : Result< Decimal< 18 >, _ > = price_mul_qty_to( price, qty, Rounding::Down );
  assert_eq!( wide, Err( RatioError::Overflow ) );
}
