//! Equal-parts splitting: even splits, remainder destinations, and the
//! non-negative kind's own edge case under an away-from-zero rounding mode.
//!
//! Every expected value below is hand-verified against `round_div`'s own
//! quotient/remainder arithmetic, not assumed — see
//! `exact_ratio/tests/ratio_and_div_round_test.rs` for the lesson this
//! follows: scaled whole-unit inputs can hide the very remainder behaviour
//! under test, so every case here uses small, exact minor counts instead.

use exact_dust::{ money_dust_remainder, money_dust_split, money_dust_split_into, qty_dust_remainder, qty_dust_split, DustError, DustTo };
use exact_kind::{ Money, Quantity };
use exact_round::Rounding;

/// 12 minor units split 4 ways divides evenly — every slot is equal and no
/// `DustTo` changes the result.
#[ test ]
fn money_dust_split_divides_evenly_with_zero_remainder()
{
  let total = Money::from_minor( 12 ).unwrap();
  let want = vec![ Money::from_minor( 3 ).unwrap(); 4 ];
  assert_eq!( money_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap(), want );
  assert_eq!( money_dust_split( total, 4, Rounding::Down, DustTo::Sink ).unwrap(), want );
  assert_eq!( money_dust_split( total, 4, Rounding::Down, DustTo::Reject ).unwrap(), want );
}

/// 11 minor units split 4 ways under `Down` leaves a remainder of 3; `First`
/// folds it into slot 0, leaving every other slot at the plain floor share.
#[ test ]
fn money_dust_split_with_a_remainder_folds_it_into_the_first_part()
{
  let total = Money::from_minor( 11 ).unwrap();
  let got = money_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap();
  let want : Vec< Money > = [ 5, 2, 2, 2 ].into_iter().map( | m | Money::from_minor( m ).unwrap() ).collect();
  assert_eq!( got, want );
}

/// Under `DustTo::Sink`, every slot holds the plain floor share — the
/// remainder is not in the output — and `money_dust_remainder` reports it
/// separately.
#[ test ]
fn money_dust_split_with_dust_to_sink_leaves_every_part_equal_and_the_remainder_queryable_separately()
{
  let total = Money::from_minor( 11 ).unwrap();
  let got = money_dust_split( total, 4, Rounding::Down, DustTo::Sink ).unwrap();
  assert_eq!( got, vec![ Money::from_minor( 2 ).unwrap(); 4 ] );
  assert_eq!( money_dust_remainder( total, 4, Rounding::Down ).unwrap(), 3 );
}

/// `DustTo::Reject` refuses a split that does not divide evenly.
#[ test ]
fn money_dust_split_with_dust_to_reject_refuses_a_nonzero_remainder()
{
  let total = Money::from_minor( 11 ).unwrap();
  assert_eq!( money_dust_split( total, 4, Rounding::Down, DustTo::Reject ), Err( DustError::Remainder ) );
}

/// `DustTo::Reject` succeeds when the split happens to divide evenly.
#[ test ]
fn money_dust_split_with_dust_to_reject_succeeds_when_it_divides_evenly()
{
  let total = Money::from_minor( 12 ).unwrap();
  let want = vec![ Money::from_minor( 3 ).unwrap(); 4 ];
  assert_eq!( money_dust_split( total, 4, Rounding::Down, DustTo::Reject ).unwrap(), want );
}

/// Splitting into zero parts is refused outright.
#[ test ]
fn money_dust_split_refuses_zero_parts()
{
  let total = Money::from_minor( 12 ).unwrap();
  assert_eq!( money_dust_split( total, 0, Rounding::Down, DustTo::First ), Err( DustError::EmptyParts ) );
}

/// The non-allocating variant writes the same shares as the allocating one.
#[ test ]
fn money_dust_split_into_writes_the_same_shares_as_the_allocating_version()
{
  let total = Money::from_minor( 11 ).unwrap();
  let want = money_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap();
  let mut out = [ Money::ZERO; 4 ];
  money_dust_split_into( total, Rounding::Down, DustTo::First, &mut out ).unwrap();
  assert_eq!( out.to_vec(), want );
}

/// 10 minor units split 4 ways under `HalfEven`: quotient 2, remainder 2,
/// `2 * |2| == 4 == d`, an exact tie, broken toward the even quotient (2) —
/// so the per-share division itself does not round away, and the full
/// remaining 2 units of leftover land on slot 0 via `DustTo::First`.
#[ test ]
fn money_dust_split_half_even_breaks_a_tie_toward_the_even_share()
{
  let total = Money::from_minor( 10 ).unwrap();
  let got = money_dust_split( total, 4, Rounding::HalfEven, DustTo::First ).unwrap();
  let want : Vec< Money > = [ 4, 2, 2, 2 ].into_iter().map( | m | Money::from_minor( m ).unwrap() ).collect();
  assert_eq!( got, want );
}

/// A non-negative kind splits cleanly under `Down` — the floor share times
/// the part count never exceeds the total, so the leftover folded into slot
/// 0 is always non-negative.
#[ test ]
fn qty_dust_split_with_down_rounding_never_goes_negative()
{
  let total = Quantity::from_minor( 11 ).unwrap();
  let got = qty_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap();
  let want : Vec< Quantity > = [ 5, 2, 2, 2 ].into_iter().map( | m | Quantity::from_minor( m ).unwrap() ).collect();
  assert_eq!( got, want );
}

/// `Up` rounds the per-share quotient away from zero: `round_div(1, 4, Up)`
/// is 1 (quotient 0, remainder 1, rounded up to 1), so 4 shares collectively
/// claim 4 minor units against a total of only 1 — a leftover of `1 - 4 =
/// -3`. Folding that negative leftover into slot 0 under `DustTo::First`
/// would need slot 0 to hold `1 + ( -3 ) = -2`, which `Quantity` refuses to
/// construct — so the split reports `Overflow` rather than silently wrapping
/// or panicking.
#[ test ]
fn qty_dust_split_refuses_a_first_slot_that_would_go_negative_under_up_rounding()
{
  let total = Quantity::from_minor( 1 ).unwrap();
  assert_eq!( qty_dust_split( total, 4, Rounding::Up, DustTo::First ), Err( DustError::Overflow ) );
}

/// `qty_dust_remainder` reports the held-back amount without touching any
/// slot — same figure as the money case, since the arithmetic is identical
/// at the minor-unit level.
#[ test ]
fn qty_dust_remainder_reports_the_held_back_amount_without_touching_any_slot()
{
  let total = Quantity::from_minor( 11 ).unwrap();
  assert_eq!( qty_dust_remainder( total, 4, Rounding::Down ).unwrap(), 3 );
}
