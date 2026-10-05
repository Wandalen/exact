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

/// More fractional digits than the scale holds is refused at every entry
/// point, never cut off — and exactly the scale's six digits is accepted.
#[ test ]
fn extra_fractional_digits_are_refused_at_every_entry_point()
{
  let refused = KindError::ExcessPrecision { digits : 7, scale : 6 };
  assert_eq!( money_from_str( "1.2345678" ), Err( refused ) );
  assert_eq!( qty_from_str( "1.2345678" ), Err( refused ) );
  assert_eq!( price_from_str( "1.2345678" ), Err( refused ) );
  assert_eq!( money_from_str( "1.234567" ).unwrap().minor(), 1_234_567 );
}

/// `-0` is zero, not a negative quantity — there is no negative zero — and a
/// leading `+` is accepted.
#[ test ]
fn minus_zero_is_zero_and_a_leading_plus_is_accepted()
{
  assert_eq!( qty_from_str( "-0" ).unwrap().minor(), 0 );
  assert_eq!( money_from_str( "+1.5" ).unwrap().minor(), 1_500_000 );
}

/// Infinity and exponent forms are refused as malformed, the same as `NaN`.
#[ test ]
fn infinity_and_exponent_forms_are_refused()
{
  for bad in [ "inf", "-inf", "1e5", "1_000" ]
  {
    let got = money_from_str( bad );
    assert!( matches!( got, Err( KindError::Malformed { .. } ) ), "{bad:?} should be malformed" );
  }
}
