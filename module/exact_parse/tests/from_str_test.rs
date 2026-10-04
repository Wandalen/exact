//! Text parsing dispatched per kind, under the preferred design's
//! free-function names — the "1.23" case the preferred design calls out
//! lives here, not in the eventual facade.

use exact_kind::KindError;
use exact_parse::{ money_from_str, price_from_str, qty_from_str };

/// `"1.23"` parses exactly through every per-kind entry point.
#[ test ]
fn one_point_two_three_parses_exactly_through_every_kind()
{
  assert_eq!( money_from_str( "1.23" ).unwrap().minor(), 1_230_000 );
  assert_eq!( qty_from_str( "1.23" ).unwrap().minor(), 1_230_000 );
  assert_eq!( price_from_str( "1.23" ).unwrap().minor(), 1_230_000 );
}

/// A quantity refuses a negative value at the free-function entry point too.
#[ test ]
fn qty_from_str_refuses_a_negative_value()
{
  assert!( matches!( qty_from_str( "-1" ), Err( KindError::Negative { .. } ) ) );
}

/// Malformed text is refused at every entry point, dispatch adding no leniency.
#[ test ]
fn malformed_text_is_refused_at_every_entry_point()
{
  assert!( money_from_str( "NaN" ).is_err() );
  assert!( money_from_str( "" ).is_err() );
  assert!( price_from_str( "NaN" ).is_err() );
  assert!( price_from_str( "" ).is_err() );
  assert!( qty_from_str( "NaN" ).is_err() );
}
