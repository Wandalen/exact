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

/// `DustTo::Reject` refuses a remainder, not a rounded share: 3.333… and
/// 6.666… round to 3.33 and 6.67, which leave nothing over. Refusing the
/// rounding itself is `Rounding::Exact`'s job.
#[ test ]
fn reject_refuses_a_remainder_not_a_rounded_share()
{
  assert_eq!( minors( &money_dust_split_weighted( cents( "10.00" ), &[ 1, 2 ], Rounding::HalfEven, DustTo::Reject ).unwrap() ), [ 333, 667 ] );
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 1, 2 ], Rounding::Down, DustTo::Reject ), Err( DustError::Remainder ) );
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 1, 1, 1 ], Rounding::HalfEven, DustTo::Reject ), Err( DustError::Remainder ) );
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 1, 2 ], Rounding::Exact, DustTo::Reject ), Err( DustError::Remainder ) );
}

/// No weights, or only zero weights, give nothing to split by.
#[ test ]
fn no_weight_to_split_by_is_empty_parts()
{
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[], Rounding::Down, DustTo::First ), Err( DustError::EmptyParts ) );
  assert_eq!( money_dust_split_weighted( cents( "10.00" ), &[ 0, 0 ], Rounding::Down, DustTo::First ), Err( DustError::EmptyParts ) );
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

/// Under `TowardZero` every part keeps the total's sign. Under a mode that
/// rounds the shares past the total, `First` takes the excess from slot 0 —
/// pinned here so the documented behaviour cannot change unnoticed.
#[ test ]
fn only_toward_zero_keeps_every_part_on_the_totals_side()
{
  let weight_lists : [ &[ u32 ]; 6 ] = [ &[ 1 ], &[ 1, 1, 1 ], &[ 70, 20, 10 ], &[ 3, 0, 7, 11 ], &[ 9999, 1 ], &[ 0, 1, 1 ] ];
  for weights in weight_lists
  {
    for total in -1000_i64 ..= 1000
    {
      let parts = money_dust_split_weighted( Money::from_minor( total ).unwrap(), weights, Rounding::TowardZero, DustTo::First ).unwrap();
      assert!( parts.iter().all( | p | p.minor() * total >= 0 ), "{total} by {weights:?}" );
    }
  }
  assert_eq!( minors( &money_dust_split_weighted( cents( "0.01" ), &[ 0, 1, 1 ], Rounding::Up, DustTo::First ).unwrap() ), [ -1, 1, 1 ] );
  assert_eq!( minors( &money_dust_split_weighted( cents( "-0.01" ), &[ 0, 1, 1 ], Rounding::Down, DustTo::First ).unwrap() ), [ 1, -1, -1 ] );
}

/// Equal weights give exactly what the equal split gives, under every mode.
#[ test ]
fn equal_weights_match_the_equal_split()
{
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven, Rounding::TowardZero, Rounding::AwayFromZero, Rounding::HalfUp, Rounding::HalfDown ]
  {
    for total in [ -1001, -10, -1, 0, 1, 10, 1001 ]
    {
      let money = Money::from_minor( total ).unwrap();
      let weighted = money_dust_split_weighted( money, &[ 5, 5, 5 ], mode, DustTo::First ).unwrap();
      let equal = money_dust_split( money, 3, mode, DustTo::First ).unwrap();
      assert_eq!( weighted, equal, "{total} under {mode:?}" );
    }
  }
}

/// With `DustTo::First` the parts always add back to exactly the total — for
/// positive and negative totals, uneven weights, and every rounding mode that
/// rounds rather than refuses. `DustTo::Sink` gives the same parts with the
/// dust held back: only slot 0 differs, by exactly the dust.
#[ test ]
fn the_parts_always_add_back_to_the_total()
{
  let weight_lists : [ &[ u32 ]; 5 ] = [ &[ 1 ], &[ 1, 1, 1 ], &[ 70, 20, 10 ], &[ 3, 0, 7, 11 ], &[ 9999, 1 ] ];
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
        let mut first = minors( &parts );
        first[ 0 ] -= dust;
        assert_eq!( first, sink, "Sink: {total} by {weights:?} under {mode:?}" );
      }
    }
  }
}

/// Large totals and large weights cannot overflow the multiply.
#[ test ]
fn large_totals_and_weights_do_not_overflow()
{
  let parts = money_dust_split_weighted( Money::MAX, &[ u32::MAX, u32::MAX ], Rounding::Down, DustTo::First ).unwrap();
  assert_eq!( minors( &parts ).iter().sum::< i64 >(), Money::MAX.minor() );
}
