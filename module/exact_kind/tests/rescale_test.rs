//! `rescale` between scales: finer is exact, coarser rounds per the mode, and
//! `Exact` refuses a dropped digit.

use exact_kind::{ Decimal, KindError, Money, Qty };
use exact_round::Rounding;

/// Going finer only adds zeros, so it is exact under every mode.
#[ test ]
fn rescaling_finer_is_exact()
{
  let cash : Decimal< 2 > = Decimal::parse( "12.34" ).unwrap();
  assert_eq!( cash.rescale::< 6 >( Rounding::Exact ).unwrap(), Money::parse( "12.34" ).unwrap() );
}

/// Going coarser rounds the dropped digits per the caller's mode, at both signs.
#[ test ]
fn rescaling_coarser_rounds_per_the_mode()
{
  let fine = Money::parse( "12.345678" ).unwrap();
  assert_eq!( fine.rescale::< 2 >( Rounding::HalfEven ).unwrap(), Decimal::< 2 >::parse( "12.35" ).unwrap() );
  assert_eq!( fine.rescale::< 2 >( Rounding::Down ).unwrap(), Decimal::< 2 >::parse( "12.34" ).unwrap() );
  let negative = Money::parse( "-1.005" ).unwrap();
  assert_eq!( negative.rescale::< 2 >( Rounding::Down ).unwrap(), Decimal::< 2 >::parse( "-1.01" ).unwrap() );
}

/// Under `Exact`, a coarser scale that would drop a digit is refused.
#[ test ]
fn exact_refuses_a_dropped_digit()
{
  let fine = Money::parse( "12.345678" ).unwrap();
  assert_eq!( fine.rescale::< 2 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 6, scale : 2 } ) );
  assert_eq!( fine.rescale::< 6 >( Rounding::Exact ).unwrap(), fine );
}

/// A quantity keeps its non-negativity: 0.12345678 BTC down to six places.
#[ test ]
fn a_quantity_rescales_like_a_decimal()
{
  let satoshis : Qty< 8 > = Qty::parse( "0.12345678" ).unwrap();
  assert_eq!( satoshis.rescale::< 6 >( Rounding::Down ).unwrap().minor(), 123_456 );
  assert_eq!( satoshis.rescale::< 6 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 8, scale : 6 } ) );
}

/// Going finer past the ceiling is refused, not wrapped, with the error that says why.
#[ test ]
fn rescaling_past_the_ceiling_is_refused()
{
  assert_eq!( Money::MAX.rescale::< 8 >( Rounding::Exact ), Err( KindError::ExceedsCeiling { minor : 900_000_000_000_000_000 } ) );
  assert_eq!( Money::MAX.rescale::< 18 >( Rounding::Exact ), Err( KindError::Overflow { operation : "rescale" } ) );
}

/// The refusal reports the digits the value has, not the scale it is held
/// at: `1.005` at scale 6 is `1.005000`, three digits, refused exactly as
/// `parse` refuses the same text at scale 2.
#[ test ]
fn exact_reports_the_digits_the_value_has()
{
  let short = Money::parse( "1.005" ).unwrap();
  assert_eq!( short.rescale::< 2 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 3, scale : 2 } ) );
  assert_eq!( short.rescale::< 2 >( Rounding::Exact ).unwrap_err(), Decimal::< 2 >::parse( "1.005" ).unwrap_err() );
  assert_eq!( Money::parse( "-0.01" ).unwrap().rescale::< 1 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 2, scale : 1 } ) );
  let tiny : Qty< 8 > = Qty::parse( "0.0000001" ).unwrap();
  assert_eq!( tiny.rescale::< 6 >( Rounding::Exact ), Err( KindError::ExcessPrecision { digits : 7, scale : 6 } ) );
}
