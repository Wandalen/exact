//! Test Matrix T01 — text in, exact value out, and the same text back.
//!
//! Round-tripping is the property that makes a rendered decimal safe to store,
//! log and re-read. It is also the property that a "close enough" numeric type
//! silently fails: `0.1` parsed into an `f64` and printed back gives `0.1`
//! because the printer rounds, not because the value survived. Here the value
//! survives, and the tests below check the parts of the grammar where an
//! implementation would be tempted to repair input instead of refusing it.

use exact_kind::{ Decimal, KindError, Money };

/// T01 — `"0.1"` parses exactly and renders back to `"0.1"`.
///
/// The single case this family is named for. Asserted on both halves: the
/// stored integer is exactly one tenth of a whole unit at scale 6, and the
/// rendering is the input rather than `0.100000`.
#[ test ]
fn a_tenth_parses_exactly_and_renders_back_unchanged()
{
  let tenth = Money::parse( "0.1" ).unwrap();
  assert_eq!( tenth.minor(), 100_000 );
  assert_eq!( tenth.to_string(), "0.1" );
}

/// T01 — round-tripping holds across signs, magnitudes and trailing zeros.
///
/// Each input is its own canonical spelling, so `parse( render( v ) ) == v` and
/// `render( parse( s ) ) == s` are both asserted in one pass.
#[ test ]
fn every_canonical_spelling_survives_the_round_trip()
{
  for text in [ "0", "1", "-1", "0.000001", "-0.000001", "1.5", "-1.5", "1.05", "0.00012", "9000000000", "123.456789" ]
  {
    let value = Money::parse( text ).unwrap();
    assert_eq!( value.to_string(), text, "rendering {text} changed it" );
    assert_eq!( Money::parse( &value.to_string() ).unwrap(), value );
  }
}

/// Non-canonical spellings parse to the right value, and render canonically.
///
/// Accepting `"1.50"` and rendering `"1.5"` is not a round-trip failure — the
/// two spell the same value, and one canonical rendering per value is what
/// makes rendered text comparable at all.
#[ test ]
fn trailing_zeros_and_a_leading_plus_are_accepted_and_normalized()
{
  assert_eq!( Money::parse( "1.50" ).unwrap(), Money::parse( "1.5" ).unwrap() );
  assert_eq!( Money::parse( "+2" ).unwrap(), Money::parse( "2" ).unwrap() );
  assert_eq!( Money::parse( "1.500000" ).unwrap().to_string(), "1.5" );
  assert_eq!( Money::parse( "-0.0" ).unwrap(), Money::ZERO );
}

/// Excess precision is refused, not rounded and not truncated.
///
/// A seventh decimal place into a scale-6 type is the caller saying something
/// the type cannot hold. Truncating loses it silently; rounding invents a value
/// nobody wrote. The error names both numbers so the caller can see which type
/// they actually wanted.
#[ test ]
fn a_digit_past_the_scale_is_an_error_rather_than_a_rounding()
{
  assert_eq!
  (
    Money::parse( "0.1234567" ),
    Err( KindError::ExcessPrecision { digits : 7, scale : 6 } ),
  );
  // The last digit the scale does hold is accepted, so the boundary is at the
  // scale and not one place inside it.
  assert!( Money::parse( "0.123456" ).is_ok() );
}

/// Text outside the grammar is refused, including every float spelling.
///
/// `NaN` and `inf` name no exact quantity; `1e6` and `1_000` name one this
/// grammar does not spell. All four are errors rather than best-effort parses,
/// because a numeric parser that guesses is how a float gets in.
///
/// Asserts the specific `KindError::Malformed` variant, not just `.is_err()` —
/// a prior version of this test only checked `.is_err()`, so a regression
/// that misrouted one of these inputs to a different error variant (say,
/// `ExcessPrecision` or `Overflow`) would have passed silently.
#[ test ]
fn float_spellings_and_malformed_text_are_all_refused()
{
  for text in [ "", "-", ".", ".5", "1.", "NaN", "inf", "-inf", "1e6", "1_000", "1.2.3", " 1", "1 ", "0x10" ]
  {
    assert!(
      matches!( Money::parse( text ), Err( KindError::Malformed { .. } ) ),
      "{text:?} should have been refused as KindError::Malformed",
    );
  }
}

/// A scale-0 decimal renders with no fractional part at all.
///
/// The degenerate scale is where an off-by-one in the renderer would show up as
/// a stray `.` or a division by `10⁰` gone wrong.
#[ test ]
fn the_zero_scale_renders_as_a_plain_integer()
{
  let whole = Decimal::< 0 >::parse( "-42" ).unwrap();
  assert_eq!( whole.minor(), -42 );
  assert_eq!( whole.to_string(), "-42" );
  assert!( Decimal::< 0 >::parse( "1.5" ).is_err() );
}

/// `.whole()` truncates toward zero on both sides of the sign.
///
/// The sign is exactly where a truncating integer division is easy to get
/// backwards — truncate-toward-zero and floor agree for every positive value
/// and disagree for every negative non-integer one, so a positive-only check
/// cannot tell them apart.
#[ test ]
fn whole_truncates_toward_zero_on_both_sides_of_the_sign()
{
  assert_eq!( Money::parse( "1.9" ).unwrap().whole(), 1 );
  assert_eq!( Money::parse( "-1.9" ).unwrap().whole(), -1 );
  assert_eq!( Money::parse( "2" ).unwrap().whole(), 2 );
  assert_eq!( Money::parse( "-2" ).unwrap().whole(), -2 );
  assert_eq!( Money::ZERO.whole(), 0 );
}
