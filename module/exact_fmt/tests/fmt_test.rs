//! Formatting per kind, and the buffer-writing primitive both go through.

use exact_fmt::{ FmtError, fmt_into, money_fmt, price_fmt, qty_fmt };
use exact_kind::{ Money, Price, Quantity };

/// Per-kind formatting renders exactly like the underlying `Display` impl.
#[ test ]
fn per_kind_formatting_matches_the_underlying_display_impl()
{
  let v = Money::parse( "1.5" ).unwrap();
  assert_eq!( money_fmt( v ), v.to_string() );
  assert_eq!( money_fmt( v ), "1.5" );

  let q = Quantity::from_int( 3 ).unwrap();
  assert_eq!( qty_fmt( q ), "3" );
}

/// `fmt_into` writes the same text into a buffer, with no allocation.
#[ test ]
fn fmt_into_writes_the_same_text_into_a_buffer()
{
  let v = Money::parse( "123.456789" ).unwrap();
  let mut buf = [ 0_u8; 32 ];
  let written = fmt_into( v, &mut buf ).unwrap();
  assert_eq!( core::str::from_utf8( &buf[ .. written ] ).unwrap(), "123.456789" );
}

/// A buffer too small to hold the rendered text is refused, not truncated.
#[ test ]
fn a_too_small_buffer_is_refused_rather_than_truncated()
{
  let v = Money::parse( "123.456789" ).unwrap();
  let mut buf = [ 0_u8; 3 ];
  assert_eq!( fmt_into( v, &mut buf ), Err( FmtError::BufFull ) );
}

/// A buffer exactly the rendered length succeeds, with no slack required.
#[ test ]
fn a_buffer_exactly_the_rendered_length_succeeds()
{
  let v = Money::parse( "1.5" ).unwrap();
  let mut buf = [ 0_u8; 3 ]; // "1.5" is exactly 3 bytes
  assert_eq!( fmt_into( v, &mut buf ).unwrap(), 3 );
}

/// A price renders like the money it wraps, a negative one included.
#[ test ]
fn price_fmt_renders_a_price_negative_included()
{
  assert_eq!( price_fmt( Price::parse( "1.25" ).unwrap() ), "1.25" );
  assert_eq!( price_fmt( Price::parse( "-0.05" ).unwrap() ), "-0.05" );
}

/// `fmt_into` renders every kind, and the sign of a negative value.
#[ test ]
fn fmt_into_renders_every_kind_and_a_negative_sign()
{
  let mut buf = [ 0_u8; 32 ];

  let written = fmt_into( Money::parse( "-12.5" ).unwrap(), &mut buf ).unwrap();
  assert_eq!( &buf[ .. written ], b"-12.5" );

  let written = fmt_into( Quantity::parse( "2.5" ).unwrap(), &mut buf ).unwrap();
  assert_eq!( &buf[ .. written ], b"2.5" );

  let written = fmt_into( Price::parse( "-0.05" ).unwrap(), &mut buf ).unwrap();
  assert_eq!( &buf[ .. written ], b"-0.05" );
}

/// A buffer that runs out partway through is still refused, even after an
/// earlier piece (here the `-`) fit.
#[ test ]
fn a_buffer_that_runs_out_partway_is_still_refused()
{
  let mut buf = [ 0_u8; 2 ]; // the "-" fits, "12" does not
  assert_eq!( fmt_into( Money::parse( "-12.5" ).unwrap(), &mut buf ), Err( FmtError::BufFull ) );
}

/// A quantity renders its fraction trimmed, and zero as plain `0`.
#[ test ]
fn qty_fmt_renders_a_fraction_and_zero()
{
  assert_eq!( qty_fmt( Quantity::parse( "2.50" ).unwrap() ), "2.5" );
  assert_eq!( qty_fmt( Quantity::ZERO ), "0" );
}

/// The smallest negative amount keeps every leading fractional zero.
#[ test ]
fn money_fmt_keeps_the_leading_zeros_of_the_smallest_amount()
{
  assert_eq!( money_fmt( Money::from_minor( -1 ).unwrap() ), "-0.000001" );
}

/// A zero-length buffer is refused, not written past.
#[ test ]
fn an_empty_buffer_is_refused()
{
  assert_eq!( fmt_into( Money::ZERO, &mut [] ), Err( FmtError::BufFull ) );
}
