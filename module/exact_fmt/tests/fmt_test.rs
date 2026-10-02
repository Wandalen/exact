//! Formatting per kind, and the buffer-writing primitive both go through.

use exact_fmt::{ FmtError, fmt_into, money_fmt, qty_fmt };
use exact_kind::{ Money, Quantity };

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
