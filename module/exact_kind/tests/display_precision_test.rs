//! `Display` honours a requested precision — `{:.2}` prints exactly two
//! digits after the point — on `Decimal`, `Qty` and `PriceAt`. Without one,
//! rendering is unchanged: exact, trailing zeros trimmed.

use exact_kind::{ Decimal, Money, Price, PriceAt, Qty, Quantity };

/// More places than the value has are filled with zeros.
#[ test ]
fn a_precision_pads_with_zeros()
{
  assert_eq!( format!( "{:.2}", Price::parse( "0.9" ).unwrap() ), "0.90" );
  assert_eq!( format!( "{:.2}", Decimal::< 2 >::parse( "8024.7" ).unwrap() ), "8024.70" );
  assert_eq!( format!( "{:.4}", Decimal::< 2 >::parse( "1.5" ).unwrap() ), "1.5000" );
  assert_eq!( format!( "{:.2}", Decimal::< 0 >::parse( "5" ).unwrap() ), "5.00" );
  assert_eq!( format!( "{:.20}", Decimal::< 2 >::parse( "1.5" ).unwrap() ), "1.50000000000000000000" );
}

/// Fewer places round the dropped digits half to even, as `{:.N}` does for `f64`.
#[ test ]
fn fewer_places_round_half_to_even()
{
  assert_eq!( format!( "{:.2}", Money::parse( "1.005" ).unwrap() ), "1.00" );
  assert_eq!( format!( "{:.2}", Money::parse( "1.015" ).unwrap() ), "1.02" );
  assert_eq!( format!( "{:.2}", Money::parse( "1.0051" ).unwrap() ), "1.01" );
  assert_eq!( format!( "{:.0}", Money::parse( "2.5" ).unwrap() ), "2" );
  assert_eq!( format!( "{:.0}", Money::parse( "3.5" ).unwrap() ), "4" );
  assert_eq!( format!( "{:.2}", Money::parse( "9.999" ).unwrap() ), "10.00" );
}

/// Negative values round the same way, and a value that rounds to zero
/// prints as `0.00`, never `-0.00`.
#[ test ]
fn negatives_round_symmetrically_and_never_print_minus_zero()
{
  assert_eq!( format!( "{:.2}", Money::parse( "-1.005" ).unwrap() ), "-1.00" );
  assert_eq!( format!( "{:.2}", Money::parse( "-1.015" ).unwrap() ), "-1.02" );
  assert_eq!( format!( "{:.2}", Money::parse( "-0.9" ).unwrap() ), "-0.90" );
  assert_eq!( format!( "{:.2}", Money::parse( "-0.004" ).unwrap() ), "0.00" );
}

/// Exactly the value's own scale prints every digit, trailing zeros kept.
#[ test ]
fn the_value_s_own_scale_keeps_its_trailing_zeros()
{
  assert_eq!( format!( "{:.6}", Money::parse( "1.5" ).unwrap() ), "1.500000" );
  assert_eq!( format!( "{:.6}", Money::MAX ), "9000000000.000000" );
  assert_eq!( format!( "{:.6}", Money::MIN ), "-9000000000.000000" );
}

/// Without a precision nothing changes: exact, trailing zeros trimmed.
#[ test ]
fn without_a_precision_rendering_is_unchanged()
{
  assert_eq!( format!( "{}", Price::parse( "0.90" ).unwrap() ), "0.9" );
  assert_eq!( format!( "{}", Money::parse( "-1.50" ).unwrap() ), "-1.5" );
  assert_eq!( Money::parse( "2" ).unwrap().to_string(), "2" );
}

/// `Qty` and `PriceAt` pass the precision through to the decimal they wrap,
/// at the standard scale and at their own.
#[ test ]
fn qty_and_price_pass_the_precision_through()
{
  assert_eq!( format!( "{:.2}", Quantity::from_int( 3 ).unwrap() ), "3.00" );
  assert_eq!( format!( "{:.3}", Quantity::parse( "0.12345" ).unwrap() ), "0.123" );
  assert_eq!( format!( "{:.2}", Price::parse( "65000" ).unwrap() ), "65000.00" );
  assert_eq!( format!( "{:.2}", PriceAt::< 2 >::parse( "65000" ).unwrap() ), "65000.00" );
  assert_eq!( format!( "{:.4}", Qty::< 8 >::parse( "0.12345679" ).unwrap() ), "0.1235" );
}

/// Rounding the text leaves the value alone: the same value renders exactly
/// again without a precision.
#[ test ]
fn a_precision_rounds_the_text_never_the_value()
{
  let v = Money::parse( "1.005" ).unwrap();
  assert_eq!( format!( "{v:.2}" ), "1.00" );
  assert_eq!( v.to_string(), "1.005" );
  assert_eq!( v.minor(), 1_005_000 );
}
