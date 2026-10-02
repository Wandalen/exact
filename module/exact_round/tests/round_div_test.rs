//! `round_div` under every rounding mode, at both signs, including the
//! divisor-normalization path.

use exact_round::{ RoundError, Rounding, round_div, round_div_wide };

/// A zero divisor is refused.
#[ test ]
fn round_div_refuses_a_zero_divisor()
{
  assert_eq!( round_div( 1, 0, Rounding::Down ), Err( RoundError::DivZero ) );
}

/// A negative divisor is normalized — the sign cases below only ever see a
/// positive one.
#[ test ]
fn round_div_normalizes_a_negative_divisor()
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
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven ]
  {
    assert_eq!( round_div( 8, 4, mode ).unwrap(), 2 );
  }
}

/// Normalizing a negative divisor overflows when the dividend is `i64::MIN` —
/// `i64::MIN.checked_neg()` has no representable result.
#[ test ]
fn round_div_reports_overflow_normalizing_i64_min_against_a_negative_divisor()
{
  assert_eq!( round_div( i64::MIN, -1, Rounding::Down ), Err( RoundError::Overflow ) );
}

/// Normalizing a negative divisor of `i64::MIN` itself overflows the same way.
#[ test ]
fn round_div_reports_overflow_normalizing_an_i64_min_divisor()
{
  assert_eq!( round_div( 1, i64::MIN, Rounding::Down ), Err( RoundError::Overflow ) );
}

/// `HalfEven` on a remainder below one half rounds toward the nearer neighbour,
/// even when that neighbour is odd.
#[ test ]
fn half_even_rounds_below_half_toward_the_nearer_neighbour()
{
  assert_eq!( round_div( 13, 4, Rounding::HalfEven ).unwrap(), 3 );   //  3.25 ->  3
  assert_eq!( round_div( -13, 4, Rounding::HalfEven ).unwrap(), -3 ); // -3.25 -> -3
}

/// `round_div_wide` agrees with `round_div` on every input both accept, so the
/// two copies of the rounding rules cannot drift apart unnoticed.
#[ test ]
fn round_div_wide_agrees_with_round_div()
{
  for n in [ -13, -8, -7, -5, -1, 0, 1, 5, 7, 8, 13, i64::MAX, i64::MIN + 1 ]
  {
    for d in [ -4, -2, 1, 2, 3, 4, i64::MAX ]
    {
      for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven ]
      {
        let narrow = round_div( n, d, mode ).map( i128::from );
        assert_eq!( round_div_wide( i128::from( n ), i128::from( d ), mode ), narrow, "{n} / {d}, {mode:?}" );
      }
    }
  }
}

/// `round_div_wide` divides a dividend no `i64` can hold — the case it exists for.
#[ test ]
fn round_div_wide_divides_a_dividend_wider_than_i64()
{
  let n = i128::from( i64::MAX ) * 3 + 1; // (3 × i64::MAX + 1) / 3 = i64::MAX + 1/3
  assert_eq!( round_div_wide( n, 3, Rounding::Down ), Ok( i128::from( i64::MAX ) ) );
  assert_eq!( round_div_wide( n, 3, Rounding::Up ), Ok( i128::from( i64::MAX ) + 1 ) );
  assert_eq!( round_div_wide( 1, 0, Rounding::Down ), Err( RoundError::DivZero ) );
}
