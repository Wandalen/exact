//! Encoding to the wire form and back, per kind, and the failure modes
//! `Wire` is specifically positioned to see: a bad discriminator, a bad
//! scale, a truncated slice, and a decoded value outside what the kind can
//! hold.

use exact_bytes::{ KIND_MONEY, KIND_PRICE, KIND_QTY, Wire, WireError, money_from_wire, money_to_wire, qty_from_wire, qty_to_wire, price_from_wire, price_to_wire };
use exact_kind::{ Money, Quantity, Price };

/// A money value round-trips through the wire form unchanged.
#[ test ]
fn money_round_trips_through_the_wire_form()
{
  let v = Money::parse( "123.456789" ).unwrap();
  let wire = money_to_wire( v );
  assert_eq!( wire.kind(), KIND_MONEY );
  assert_eq!( money_from_wire( wire ).unwrap(), v );
}

/// A quantity round-trips through the wire form unchanged.
#[ test ]
fn qty_round_trips_through_the_wire_form()
{
  let v = Quantity::from_int( 7 ).unwrap();
  let wire = qty_to_wire( v );
  assert_eq!( wire.kind(), KIND_QTY );
  assert_eq!( qty_from_wire( wire ).unwrap(), v );
}

/// A price round-trips through the wire form unchanged.
#[ test ]
fn price_round_trips_through_the_wire_form()
{
  let v = Price::parse( "9.99" ).unwrap();
  let wire = price_to_wire( v );
  assert_eq!( wire.kind(), KIND_PRICE );
  assert_eq!( price_from_wire( wire ).unwrap(), v );
}

/// A money-encoded wire is refused by the quantity decoder, and vice versa.
#[ test ]
fn a_wire_encoded_as_one_kind_is_refused_by_another_kind_s_decoder()
{
  let money_wire = money_to_wire( Money::parse( "1" ).unwrap() );
  assert_eq!( qty_from_wire( money_wire ), Err( WireError::BadKind ) );

  let qty_wire = qty_to_wire( Quantity::from_int( 1 ).unwrap() );
  assert_eq!( money_from_wire( qty_wire ), Err( WireError::BadKind ) );
}

/// A wrong scale byte is refused, independent of the kind byte being correct.
#[ test ]
fn a_mismatched_scale_byte_is_refused()
{
  let wire = money_to_wire( Money::parse( "1" ).unwrap() );
  let mut bytes = wire.to_bytes();
  bytes[ 8 ] = 9; // a scale this family never uses
  let tampered = Wire::from_bytes( &bytes ).unwrap();
  assert_eq!( money_from_wire( tampered ), Err( WireError::BadScale ) );
}

/// A byte slice shorter than the encoded length is refused, not padded.
#[ test ]
fn a_short_byte_slice_is_refused_as_truncated()
{
  let wire = money_to_wire( Money::parse( "1" ).unwrap() );
  let bytes = wire.to_bytes();
  assert_eq!( Wire::from_bytes( &bytes[ .. Wire::ENCODED_LEN - 1 ] ), Err( WireError::Truncated ) );
}

/// Round-tripping through raw bytes, not only through the `Wire` struct,
/// preserves the value.
#[ test ]
fn round_tripping_through_raw_bytes_preserves_the_value()
{
  let v = Money::parse( "-42.5" ).unwrap();
  let bytes = money_to_wire( v ).to_bytes();
  let decoded = Wire::from_bytes( &bytes ).unwrap();
  assert_eq!( money_from_wire( decoded ).unwrap(), v );
}

/// A decoded minor count past the declared ceiling is refused as overflow,
/// not silently accepted — `Wire`'s `minor` is a bare `i64` with no range
/// check of its own until a kind's constructor sees it.
#[ test ]
fn a_decoded_value_past_the_ceiling_is_refused_as_overflow()
{
  let past_ceiling = Wire::new( Money::MAX.minor() + 1, 6, KIND_MONEY );
  assert_eq!( money_from_wire( past_ceiling ), Err( WireError::Overflow ) );
}

/// A decoded negative minor count is refused for a quantity, not for money.
#[ test ]
fn a_decoded_negative_value_is_refused_only_for_a_non_negative_kind()
{
  let negative = Wire::new( -1, 6, KIND_QTY );
  assert!( matches!( qty_from_wire( negative ), Err( WireError::Negative { minor : -1 } ) ) );

  let money_negative = Wire::new( -1, 6, KIND_MONEY );
  assert!( money_from_wire( money_negative ).is_ok() );
}

/// A negative price survives the wire, and a price decoder refuses a record
/// written as another kind — in both directions.
#[ test ]
fn a_negative_price_round_trips_and_kinds_never_cross_decoders()
{
  let v = Price::parse( "-0.05" ).unwrap();
  assert_eq!( price_from_wire( price_to_wire( v ) ).unwrap(), v );
  assert_eq!( price_from_wire( money_to_wire( Money::parse( "1" ).unwrap() ) ), Err( WireError::BadKind ) );
  assert_eq!( money_from_wire( price_to_wire( v ) ), Err( WireError::BadKind ) );
}

/// A kind byte no kind uses is refused by every decoder.
#[ test ]
fn an_unknown_kind_byte_is_refused_by_every_decoder()
{
  let unknown = Wire::new( 0, 6, 7 );
  assert_eq!( money_from_wire( unknown ), Err( WireError::BadKind ) );
  assert_eq!( qty_from_wire( unknown ), Err( WireError::BadKind ) );
  assert_eq!( price_from_wire( unknown ), Err( WireError::BadKind ) );
}

/// An encoded record carries the value's minor count, the family's scale,
/// and its kind — the three fields a decoder checks.
#[ test ]
fn an_encoded_record_carries_minor_scale_and_kind()
{
  let wire = money_to_wire( Money::parse( "1.5" ).unwrap() );
  assert_eq!( ( wire.minor(), wire.scale(), wire.kind() ), ( 1_500_000, 6, KIND_MONEY ) );
}

/// A decoded count one unit below the negative ceiling is refused too.
#[ test ]
fn a_decoded_value_below_the_negative_ceiling_is_refused_as_overflow()
{
  let below = Wire::new( Money::MIN.minor() - 1, 6, KIND_MONEY );
  assert_eq!( money_from_wire( below ), Err( WireError::Overflow ) );
}

/// An empty slice is truncated; a longer one decodes its first record and
/// ignores the rest — only a shorter slice is refused.
#[ test ]
fn an_empty_slice_is_truncated_and_a_longer_one_decodes_its_first_record()
{
  assert_eq!( Wire::from_bytes( &[] ), Err( WireError::Truncated ) );
  let wire = money_to_wire( Money::parse( "2" ).unwrap() );
  let mut longer = wire.to_bytes().to_vec();
  longer.push( 0xff );
  assert_eq!( Wire::from_bytes( &longer ).unwrap(), wire );
}
