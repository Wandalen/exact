//! Weighted splitting: shares in proportion to whole-number weights, the
//! remainder sent to the same `DustTo` destinations as the equal split, and
//! the total conserved exactly under every rounding mode.

use exact_dust::{ money_dust_split, money_dust_split_weighted, DustError, DustTo };
use exact_kind::{ Decimal, Money };
use exact_round::Rounding;

fn cents( text : &str ) -> Decimal< 2 >
{
  Decimal::parse( text ).unwrap()
}

fn minors< const S : u32 >( parts : &[ Decimal< S > ] ) -> Vec< i64 >
{
  parts.iter().map( | p | p.minor() ).collect()
}

/// Percentages that divide evenly need no dust: `$10.00` at 70/20/10.
#[ test ]
fn weights_that_divide_evenly_split_exactly()
{
  let parts = money_dust_split_weighted( cents( "10.00" ), &[ 70, 20, 10 ], Rounding::Down, DustTo::Reject ).unwrap();
  assert_eq!( minors( &parts ), [ 700, 200, 100 ] );
}

/// Uneven shares round under the mode, and the leftover cent goes where
/// `DustTo` says: into the first slot, held back, or refused.
#[ test ]
fn the_leftover_goes_where_dust_to_says()
{
  let ten = cents( "10.00" );
  let first = money_dust_split_weighted( ten, &[ 1, 1, 1 ], Rounding::Down, DustTo::First ).unwrap();
  assert_eq!( minors( &first ), [ 334, 333, 333 ] );

  let sink = money_dust_split_weighted( ten, &[ 1, 1, 1 ], Rounding::Down, DustTo::Sink ).unwrap();
  assert_eq!( minors( &sink ), [ 333, 333, 333 ] );
  assert_eq!( ten.minor() - minors( &sink ).iter().sum::< i64 >(), 1, "the held-back dust is the total minus the parts" );

  let reject = money_dust_split_weighted( ten, &[ 1, 1, 1 ], Rounding::Down, DustTo::Reject );
  assert_eq!( reject, Err( DustError::Remainder ) );
}

/// `Rounding::Exact` refuses a share that does not divide evenly, and allows
/// one that does.
#[ test ]
fn exact_refuses_an_uneven_share()
{
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 1, 2 ], Rounding::Exact, DustTo::First ), Err( DustError::Remainder ) );
  assert_eq!( minors( &money_dust_split_weighted( cents( "9.00" ), &[ 1, 2 ], Rounding::Exact, DustTo::First ).unwrap() ), [ 300, 600 ] );
}

/// `DustTo::Reject` refuses any remainder. The shares are rounded toward
/// zero, so a remainder is left exactly when a share is not whole — under
/// every mode, `HalfEven` included: 3.333… and 6.666… become 3.33 and 6.66,
/// a cent short.
#[ test ]
fn reject_refuses_any_remainder_under_every_mode()
{
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven, Rounding::TowardZero, Rounding::AwayFromZero, Rounding::HalfUp, Rounding::HalfDown, Rounding::Exact ]
  {
    assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 1, 2 ], mode, DustTo::Reject ), Err( DustError::Remainder ), "{mode:?}" );
    assert_eq!( minors( &money_dust_split_weighted( cents( "9.00" ), &[ 1, 2 ], mode, DustTo::Reject ).unwrap() ), [ 300, 600 ], "{mode:?}" );
  }
}

/// No weights, or only zero weights, give nothing to split by.
#[ test ]
fn no_weight_to_split_by_is_empty_parts()
{
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[], Rounding::Down, DustTo::First ), Err( DustError::EmptyParts ) );
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 0, 0 ], Rounding::Down, DustTo::First ), Err( DustError::EmptyParts ) );
  assert_eq!( DustError::EmptyParts.to_string(), "nothing to split into: no parts, or no nonzero weight" );
}

/// A zero weight gets a zero share — unless `DustTo::First` sends the dust to it.
#[ test ]
fn a_zero_weight_gets_a_zero_share()
{
  let sink = money_dust_split_weighted( cents( "10.00" ), &[ 0, 1, 2 ], Rounding::Down, DustTo::Sink ).unwrap();
  assert_eq!( minors( &sink ), [ 0, 333, 666 ] );
  let first = money_dust_split_weighted( cents( "10.00" ), &[ 0, 1, 2 ], Rounding::Down, DustTo::First ).unwrap();
  assert_eq!( minors( &first ), [ 1, 333, 666 ], "the first slot takes the dust whatever its weight" );
}

/// Under every mode, with `First` and with `Sink`, every part keeps the
/// total's sign and the parts never add up past the total — checked against
/// the total itself, not against another split's output.
#[ test ]
fn no_part_and_no_sum_passes_the_total_under_any_mode()
{
  let weight_lists : [ &[ u64 ]; 6 ] = [ &[ 1 ], &[ 1, 1 ], &[ 1, 1, 1 ], &[ 70, 20, 10 ], &[ 3, 0, 7, 11 ], &[ 0, 1, 1 ] ];
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven, Rounding::TowardZero, Rounding::AwayFromZero, Rounding::HalfUp, Rounding::HalfDown ]
  {
    for to in [ DustTo::First, DustTo::Sink ]
    {
      for weights in weight_lists
      {
        for total in -1000_i64 ..= 1000
        {
          let parts = minors( &money_dust_split_weighted( Money::from_minor( total ).unwrap(), weights, mode, to ).unwrap() );
          let sum : i64 = parts.iter().sum();
          assert!( parts.iter().all( | p | p * total >= 0 ), "{total} by {weights:?} under {mode:?}, {to:?}: {parts:?}" );
          assert!( sum * total >= 0 && sum.abs() <= total.abs(), "{total} by {weights:?} under {mode:?}, {to:?}: sum {sum}" );
        }
      }
    }
  }
  // The reviewers' cases: a zero-weight party is never charged, and `Sink`
  // never pays out more than it holds.
  assert_eq!( minors( &money_dust_split_weighted( cents( "0.03" ), &[ 0, 1, 1 ], Rounding::HalfEven, DustTo::First ).unwrap() ), [ 1, 1, 1 ] );
  assert_eq!( minors( &money_dust_split_weighted( cents( "0.01" ), &[ 0, 1, 1 ], Rounding::Up, DustTo::First ).unwrap() ), [ 1, 0, 0 ] );
  assert_eq!( minors( &money_dust_split_weighted( cents( "0.03" ), &[ 1, 1 ], Rounding::HalfEven, DustTo::Sink ).unwrap() ), [ 1, 1 ] );
  assert_eq!( minors( &money_dust_split_weighted( cents( "0.01" ), &[ 1, 1 ], Rounding::Up, DustTo::Sink ).unwrap() ), [ 0, 0 ] );
}

/// Equal weights give exactly what the equal split gives when it rounds
/// toward zero — the weighted split rounds its shares that way under every
/// mode — and the same refusal under `Exact`.
#[ test ]
fn equal_weights_match_the_equal_split_toward_zero()
{
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven, Rounding::TowardZero, Rounding::AwayFromZero, Rounding::HalfUp, Rounding::HalfDown, Rounding::Exact ]
  {
    let equal_mode = if matches!( mode, Rounding::Exact ) { Rounding::Exact } else { Rounding::TowardZero };
    for total in [ -1001, -10, -1, 0, 1, 10, 1001 ]
    {
      let money = Money::from_minor( total ).unwrap();
      let weighted = money_dust_split_weighted( money, &[ 5, 5, 5 ], mode, DustTo::First );
      let equal = money_dust_split( money, 3, equal_mode, DustTo::First );
      assert_eq!( weighted, equal, "{total} under {mode:?}" );
    }
  }
}

/// With `DustTo::First` the parts always add back to exactly the total — for
/// positive and negative totals, uneven weights, and every rounding mode that
/// rounds rather than refuses. With `DustTo::Sink` the held-back dust is
/// checked against the total itself: on the total's side of zero and never
/// larger than it, so the parts never pay out more than the total. Only then
/// is it compared with `First`: the same parts, slot 0 short by exactly the
/// dust.
#[ test ]
fn the_parts_always_add_back_to_the_total()
{
  let weight_lists : [ &[ u64 ]; 5 ] = [ &[ 1 ], &[ 1, 1, 1 ], &[ 70, 20, 10 ], &[ 3, 0, 7, 11 ], &[ 9999, 1 ] ];
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven, Rounding::TowardZero, Rounding::AwayFromZero, Rounding::HalfUp, Rounding::HalfDown ]
  {
    for weights in weight_lists
    {
      for total in -1000_i64 ..= 1000
      {
        let money = Money::from_minor( total ).unwrap();
        let parts = money_dust_split_weighted( money, weights, mode, DustTo::First ).unwrap();
        assert_eq!( parts.len(), weights.len() );
        assert_eq!( minors( &parts ).iter().sum::< i64 >(), total, "{total} by {weights:?} under {mode:?}" );
        let sink = minors( &money_dust_split_weighted( money, weights, mode, DustTo::Sink ).unwrap() );
        let dust = total - sink.iter().sum::< i64 >();
        assert!( dust * total >= 0 && dust.abs() <= total.abs(), "Sink dust {dust}: {total} by {weights:?} under {mode:?}" );
        let mut first = minors( &parts );
        first[ 0 ] -= dust;
        assert_eq!( first, sink, "Sink: {total} by {weights:?} under {mode:?}" );
      }
    }
  }
}

/// Large totals and large weights cannot overflow the multiply, and weights
/// whose sum is past `u64::MAX` are still summed exactly.
#[ test ]
fn large_totals_and_weights_do_not_overflow()
{
  let parts = money_dust_split_weighted( Money::MAX, &[ u64::MAX, u64::MAX ], Rounding::Down, DustTo::First ).unwrap();
  assert_eq!( minors( &parts ).iter().sum::< i64 >(), Money::MAX.minor() );
  let parts = money_dust_split_weighted( Money::MIN, &[ u64::MAX, 1, u64::MAX ], Rounding::Up, DustTo::First ).unwrap();
  assert_eq!( minors( &parts ).iter().sum::< i64 >(), Money::MIN.minor() );
  assert!( minors( &parts ).iter().all( | &p | p <= 0 ), "a negative total gives no positive part" );
}

/// Weights can be amounts in minor units: a fee spread over fills by what
/// each fill was worth. `$10 000`, `$5 000` and `$1 000` at scale 6 are past
/// `u32::MAX`, and a `$16.00` fee splits into `$10.00`, `$5.00` and `$1.00`.
#[ test ]
fn notional_weights_past_u32_split_a_fee()
{
  let notionals = [ 10_000_000_000_u64, 5_000_000_000, 1_000_000_000 ];
  assert!( notionals[ 0 ] > u64::from( u32::MAX ) );
  let fee = Money::parse( "16" ).unwrap();
  let parts = money_dust_split_weighted( fee, &notionals, Rounding::TowardZero, DustTo::First ).unwrap();
  assert_eq!( minors( &parts ), [ 10_000_000, 5_000_000, 1_000_000 ] );
}
