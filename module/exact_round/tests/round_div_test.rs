//! `round_div` and `round_div_wide` under every rounding mode, at both signs,
//! including the minimum value as either operand.

use exact_round::{ RoundError, Rounding, round_div, round_div_wide };

/// A zero divisor is refused.
#[ test ]
fn round_div_refuses_a_zero_divisor()
{
  assert_eq!( round_div( 1, 0, Rounding::Down ), Err( RoundError::DivZero ) );
}

/// A negative divisor rounds exactly as negating both operands would.
#[ test ]
fn a_negative_divisor_rounds_like_negating_both_operands()
{
  assert_eq!( round_div( 7, -2, Rounding::Down ).unwrap(), round_div( -7, 2, Rounding::Down ).unwrap() );
}

/// `Down` rounds toward negative infinity at both signs.
#[ test ]
fn down_rounds_toward_negative_infinity()
{
  assert_eq!( round_div( 7, 2, Rounding::Down ).unwrap(), 3 );
  assert_eq!( round_div( -7, 2, Rounding::Down ).unwrap(), -4 );
}

/// `Up` rounds toward positive infinity at both signs.
#[ test ]
fn up_rounds_toward_positive_infinity()
{
  assert_eq!( round_div( 7, 2, Rounding::Up ).unwrap(), 4 );
  assert_eq!( round_div( -7, 2, Rounding::Up ).unwrap(), -3 );
}

/// `HalfEven` rounds an exact tie to the even neighbour, both directions.
#[ test ]
fn half_even_rounds_an_exact_tie_to_even()
{
  assert_eq!( round_div( 7, 2, Rounding::HalfEven ).unwrap(), 4 ); // 3.5 -> 4
  assert_eq!( round_div( -7, 2, Rounding::HalfEven ).unwrap(), -4 ); // -3.5 -> -4
  assert_eq!( round_div( 5, 2, Rounding::HalfEven ).unwrap(), 2 ); // 2.5 -> 2
}

/// `HalfEven` on a non-tying remainder rounds to the nearer neighbour.
#[ test ]
fn half_even_rounds_a_non_tie_to_the_nearest_neighbour()
{
  assert_eq!( round_div( 11, 4, Rounding::HalfEven ).unwrap(), 3 ); // 2.75 -> 3
  assert_eq!( round_div( 10, 4, Rounding::HalfEven ).unwrap(), 2 ); // 2.5 -> 2 (even)
}

/// An exact division, with no remainder, agrees across every rounding mode.
#[ test ]
fn an_exact_division_agrees_across_every_rounding_mode()
{
  for mode in EVERY_MODE
  {
    assert_eq!( round_div( 8, 4, mode ).unwrap(), 2 );
  }
}

/// The one overflow: `i64::MIN / -1`, whose quotient is one past `i64::MAX`.
#[ test ]
fn round_div_reports_overflow_only_when_the_quotient_does_not_fit()
{
  for mode in EVERY_MODE
  {
    assert_eq!( round_div( i64::MIN, -1, mode ), Err( RoundError::Overflow ) );
  }
}

/// The minimum value divides like any other value, as dividend or divisor.
///
/// Root Cause: `round_div` handled a negative divisor by negating both
/// operands first. `i64::MIN` has no positive counterpart, so the negation
/// failed and `(0, MIN)`, `(1, MIN)`, `(MIN, -2)` and `(MIN, MIN)` returned
/// `Overflow`, though each quotient fits.
///
/// Why Not Caught: the tests covered `MIN / -1`, the one real overflow, and
/// ordinary values, but no other division with `MIN` on either side.
///
/// Fix Applied: one division body, `round_div_wide`, keeps the operands'
/// signs and reads the rounding direction from the signs of the remainder
/// and divisor; `round_div` calls it.
///
/// Prevention: this test and `round_div_wide_handles_the_minimum_value_on_either_side`
/// pin each minimum-value case at both widths, in every mode.
///
/// Pitfall: two's-complement `MIN` has no positive counterpart —
/// normalising signs by negation fails exactly at the edge a test grid
/// rarely reaches.
#[ test ]
fn round_div_handles_the_minimum_value_on_either_side()
{
  for mode in EVERY_MODE
  {
    assert_eq!( round_div( 0, i64::MIN, mode ), Ok( 0 ) );
    assert_eq!( round_div( i64::MIN, -2, mode ), Ok( 1 << 62 ) );
    assert_eq!( round_div( i64::MIN, i64::MIN, mode ), Ok( 1 ) );
  }
  assert_eq!( round_div( 1, i64::MIN, Rounding::Down ), Ok( -1 ) ); // just below zero
  assert_eq!( round_div( 1, i64::MIN, Rounding::Up ), Ok( 0 ) );
  assert_eq!( round_div( 1, i64::MIN, Rounding::HalfEven ), Ok( 0 ) );
}

/// `HalfEven` on a remainder below one half rounds toward the nearer neighbour,
/// even when that neighbour is odd.
#[ test ]
fn half_even_rounds_below_half_toward_the_nearer_neighbour()
{
  assert_eq!( round_div( 13, 4, Rounding::HalfEven ).unwrap(), 3 );   //  3.25 ->  3
  assert_eq!( round_div( -13, 4, Rounding::HalfEven ).unwrap(), -3 ); // -3.25 -> -3
}

/// Every mode matches its definition on a grid of small operands, both signs:
/// `Down` is the largest integer at most `n / d`, `Up` the smallest at least
/// `n / d`, `TowardZero` and `AwayFromZero` whichever of those two is nearer
/// to and farther from zero, the three `Half*` modes the nearest, a tie
/// going to the even one, away from zero, or toward zero, and `Exact` the
/// quotient itself when `d` divides `n`, and `Inexact` otherwise.
#[ test ]
fn every_mode_matches_its_definition_on_a_grid()
{
  for n in -60_i64..=60
  {
    for d in ( -13_i64..=13 ).filter( | &d | d != 0 )
    {
      let at_most = | k : i64 | if d > 0 { k * d <= n } else { k * d >= n }; // k <= n / d
      let down = ( -61..=61 ).filter( | &k | at_most( k ) ).max().unwrap();
      let up = if down * d == n { down } else { down + 1 };
      let distance = | k : i64 | ( n - k * d ).abs();
      // `down` and `up` straddle `n / d`: at or above zero `down` is nearer zero, below it `up` is.
      let toward_zero = if down >= 0 { down } else { up };
      let away_from_zero = if down >= 0 { up } else { down };
      let nearest = | tie : i64 | match distance( down ).cmp( &distance( up ) )
      {
        core::cmp::Ordering::Less => down,
        core::cmp::Ordering::Greater => up,
        core::cmp::Ordering::Equal => tie,
      };
      let half_even = nearest( if down % 2 == 0 { down } else { up } );
      let expected =
      [
        ( Rounding::Down, down ),
        ( Rounding::Up, up ),
        ( Rounding::HalfEven, half_even ),
        ( Rounding::TowardZero, toward_zero ),
        ( Rounding::AwayFromZero, away_from_zero ),
        ( Rounding::HalfUp, nearest( away_from_zero ) ),
        ( Rounding::HalfDown, nearest( toward_zero ) ),
      ];
      for ( mode, want ) in expected
      {
        assert_eq!( round_div( n, d, mode ), Ok( want ), "{n} / {d}, {mode:?}" );
        let wide = round_div_wide( i128::from( n ), i128::from( d ), mode );
        assert_eq!( wide, Ok( i128::from( want ) ), "wide {n} / {d}, {mode:?}" );
      }
      let exact = if down * d == n { Ok( down ) } else { Err( RoundError::Inexact ) };
      assert_eq!( round_div( n, d, Rounding::Exact ), exact, "{n} / {d}, Exact" );
      let wide = round_div_wide( i128::from( n ), i128::from( d ), Rounding::Exact );
      assert_eq!( wide, exact.map( i128::from ), "wide {n} / {d}, Exact" );
    }
  }
}

/// Every mode on positive and negative quotients, off a tie and on one, with
/// the expected values worked by hand.
#[ test ]
fn every_mode_rounds_as_named()
{
  use Rounding::*;
  let modes = [ TowardZero, AwayFromZero, Down, Up, HalfEven, HalfUp, HalfDown ];
  //  n,  d,   TowardZero, AwayFromZero, Down, Up, HalfEven, HalfUp, HalfDown
  let cases : [ ( i64, i64, [ i64; 7 ] ); 9 ] =
  [
    (  7,  2, [  3,  4,  3,  4,  4,  4,  3 ] ), //  3.5
    ( -7,  2, [ -3, -4, -4, -3, -4, -4, -3 ] ), // -3.5
    (  7, -2, [ -3, -4, -4, -3, -4, -4, -3 ] ), // -3.5, negative divisor
    (  5,  2, [  2,  3,  2,  3,  2,  3,  2 ] ), //  2.5
    ( -5,  2, [ -2, -3, -3, -2, -2, -3, -2 ] ), // -2.5
    (  7,  3, [  2,  3,  2,  3,  2,  2,  2 ] ), //  2.33
    ( -8,  3, [ -2, -3, -3, -2, -3, -3, -3 ] ), // -2.67
    ( -1,  3, [  0, -1, -1,  0,  0,  0,  0 ] ), // -0.33
    (  6,  2, [  3,  3,  3,  3,  3,  3,  3 ] ), //  exact
  ];
  for ( n, d, expected ) in cases
  {
    for ( mode, want ) in modes.into_iter().zip( expected )
    {
      assert_eq!( round_div( n, d, mode ), Ok( want ), "{n} / {d} under {mode:?}" );
    }
  }
}

/// `Exact` returns a quotient with no remainder, at either sign, and refuses
/// any remainder — a tie or not, below one or above — as `Inexact`, never
/// rounding it either way.
#[ test ]
fn exact_returns_an_exact_quotient_and_refuses_any_remainder()
{
  assert_eq!( round_div( 6, 2, Rounding::Exact ), Ok( 3 ) );
  assert_eq!( round_div( -6, 2, Rounding::Exact ), Ok( -3 ) );
  assert_eq!( round_div( 6, -2, Rounding::Exact ), Ok( -3 ) );
  assert_eq!( round_div( 0, 5, Rounding::Exact ), Ok( 0 ) );
  for ( n, d ) in [ ( 7, 2 ), ( -7, 2 ), ( 7, -2 ), ( 7, 3 ), ( -1, 3 ), ( 1, 1_000_000 ) ]
  {
    assert_eq!( round_div( n, d, Rounding::Exact ), Err( RoundError::Inexact ), "{n} / {d}" );
  }
}

/// `Exact` at `i128` width: a dividend no `i64` can hold divides when it is a
/// multiple, and is refused one unit past it.
#[ test ]
fn round_div_wide_under_exact_refuses_a_remainder_past_i64()
{
  let n = i128::from( i64::MAX ) * 3;
  assert_eq!( round_div_wide( n, 3, Rounding::Exact ), Ok( i128::from( i64::MAX ) ) );
  assert_eq!( round_div_wide( n + 1, 3, Rounding::Exact ), Err( RoundError::Inexact ) );
  assert_eq!( round_div_wide( n + 1, 0, Rounding::Exact ), Err( RoundError::DivZero ) );
}

/// `round_div_wide` divides a dividend no `i64` can hold — the case it exists for.
#[ test ]
fn round_div_wide_divides_a_dividend_wider_than_i64()
{
  let n = i128::from( i64::MAX ) * 3 + 1; // (3 × i64::MAX + 1) / 3 = i64::MAX + 1/3
  assert_eq!( round_div_wide( n, 3, Rounding::Down ), Ok( i128::from( i64::MAX ) ) );
  assert_eq!( round_div_wide( n, 3, Rounding::Up ), Ok( i128::from( i64::MAX ) + 1 ) );
  assert_eq!( round_div_wide( n, 3, Rounding::HalfEven ), Ok( i128::from( i64::MAX ) ) );
  // a negative divisor: -(i64::MAX + 1/3)
  assert_eq!( round_div_wide( n, -3, Rounding::Down ), Ok( -i128::from( i64::MAX ) - 1 ) );
  assert_eq!( round_div_wide( n, -3, Rounding::Up ), Ok( -i128::from( i64::MAX ) ) );
  assert_eq!( round_div_wide( n, -3, Rounding::HalfEven ), Ok( -i128::from( i64::MAX ) ) );
  assert_eq!( round_div_wide( 1, 0, Rounding::Down ), Err( RoundError::DivZero ) );
}

/// `round_div_wide` handles `i128`'s minimum value on either side; only
/// `i128::MIN / -1` overflows.
///
/// Root Cause: `round_div_wide` began as a copy of `round_div`'s body at
/// `i128` width, including its handling of a negative divisor by negating
/// both operands first. `i128::MIN` has no positive counterpart, so
/// `(0, MIN)`, `(1, MIN)`, `(MIN, -2)` and `(MIN, MIN)` returned `Overflow`,
/// though each quotient fits.
///
/// Why Not Caught: its first tests covered a dividend wider than `i64` and
/// a zero divisor, but no division with `i128::MIN` on either side.
///
/// Fix Applied: one division body, this function, keeps the operands' signs
/// and reads the rounding direction from the signs of the remainder and
/// divisor; `round_div` calls it instead of keeping a second copy.
///
/// Prevention: this test pins each minimum-value case in every mode, and
/// `round_div_handles_the_minimum_value_on_either_side` pins the same cases
/// at `i64` width through `round_div`.
///
/// Pitfall: two's-complement `MIN` has no positive counterpart —
/// normalising signs by negation fails exactly at the edge a test grid
/// rarely reaches, and a copied body copies the edge case with it.
#[ test ]
fn round_div_wide_handles_the_minimum_value_on_either_side()
{
  for mode in EVERY_MODE
  {
    assert_eq!( round_div_wide( 0, i128::MIN, mode ), Ok( 0 ) );
    assert_eq!( round_div_wide( i128::MIN, -2, mode ), Ok( 1 << 126 ) );
    assert_eq!( round_div_wide( i128::MIN, i128::MIN, mode ), Ok( 1 ) );
    assert_eq!( round_div_wide( i128::MIN, -1, mode ), Err( RoundError::Overflow ) );
  }
  assert_eq!( round_div_wide( 1, i128::MIN, Rounding::Down ), Ok( -1 ) );
  assert_eq!( round_div_wide( 1, i128::MIN, Rounding::Up ), Ok( 0 ) );
}

/// The overflow message names its real cause: a quotient past the integer type.
#[ test ]
fn the_overflow_message_names_the_real_cause()
{
  let error = round_div( i64::MIN, -1, Rounding::Down ).unwrap_err();
  assert_eq!( error.to_string(), "the quotient does not fit the integer type" );
}

/// The inexact message names the refused remainder and the mode that refused it.
#[ test ]
fn the_inexact_message_names_the_refused_remainder()
{
  let error = round_div( 7, 2, Rounding::Exact ).unwrap_err();
  assert_eq!( error.to_string(), "the division left a remainder and Rounding::Exact was requested" );
}

/// The zero-divisor message names the zero divisor.
#[ test ]
fn the_div_zero_message_names_the_zero_divisor()
{
  assert_eq!( RoundError::DivZero.to_string(), "a zero divisor was supplied" );
}

/// Every rounding mode, for the tests that must hold under each of them.
const EVERY_MODE : [ Rounding; 8 ] =
[
  Rounding::Down,
  Rounding::Up,
  Rounding::HalfEven,
  Rounding::TowardZero,
  Rounding::AwayFromZero,
  Rounding::HalfUp,
  Rounding::HalfDown,
  Rounding::Exact,
];
