//! Ported from `exact_audit/tests/conservation_test.rs`'s T07/T08 coverage
//! for the plain-log [`Entry`]/[`verify`] auditor, plus new coverage for the
//! typed `exact_add`-backed convenience layer this crate adds on top.
//!
//! Not ported: `the_manifest_declares_no_dependencies` (T09) — this crate's
//! manifest is deliberately no longer empty; see the disclosed deviation in
//! `src/lib.rs`'s module doc comment. `the_overflow_error_names_where_it_happened`
//! is replaced by `the_overflow_error_names_the_representable_range` below,
//! since `ConservationError::Overflow` carries no `at_entry` field.

use exact_conserve::{ money_conserve_into, money_sum_assert_zero, qty_conserve_into, qty_sum_assert_zero, ConservationError, Entry, Report, verify };
use exact_kind::{ Money, Quantity };

/// A transfer, as the two postings it really is.
fn transfer( from : &str, to : &str, amount_minor : i64 ) -> Vec< Entry >
{
  vec![ Entry::new( from, -amount_minor ), Entry::new( to, amount_minor ) ]
}

/// T07 — a log whose credits match its debits reports balanced.
#[ test ]
fn a_log_of_matched_postings_balances()
{
  let mut log = transfer( "buyer", "seller", 1_250_000 );
  log.extend( transfer( "seller", "carrier", 90_000 ) );
  log.extend( transfer( "carrier", "buyer", 90_000 ) );

  let report = verify( &log ).unwrap();

  assert!( report.is_balanced() );
  assert_eq!( report.discrepancy_minor(), 0 );
  assert_eq!( report.entries, 6 );
}

/// T07 — an empty log balances, and says so without pretending it audited anything.
#[ test ]
fn an_empty_log_balances_at_zero_entries()
{
  let report = verify( &[] ).unwrap();

  assert!( report.is_balanced() );
  assert_eq!( report.entries, 0 );
}

/// T08 — one minor unit short is detected, and the shortfall is named.
#[ test ]
fn a_single_minor_unit_of_leakage_is_detected_and_named()
{
  let log = [ Entry::new( "buyer", -1_000_000 ), Entry::new( "seller", 999_999 ) ];
  let report = verify( &log ).unwrap();

  assert!( !report.is_balanced() );
  assert_eq!( report.discrepancy_minor(), -1 );

  let forged = [ Entry::new( "buyer", -1_000_000 ), Entry::new( "seller", 1_000_001 ) ];
  assert_eq!( verify( &forged ).unwrap().discrepancy_minor(), 1 );
}

/// T08 — a single unit stays visible in a log large enough to hide it.
#[ test ]
fn one_unit_stays_visible_against_a_million_units_of_turnover()
{
  let mut log : Vec< Entry > = ( 0..1_000 ).flat_map( | i | transfer( "a", "b", i64::from( i ) * 1_000 ) ).collect();
  log.push( Entry::new( "leak", -1 ) );

  let report = verify( &log ).unwrap();

  assert_eq!( report.entries, 2_001 );
  assert_eq!( report.discrepancy_minor(), -1 );
}

/// The report renders the two outcomes distinguishably.
#[ test ]
fn the_report_renders_both_outcomes_in_words()
{
  assert_eq!( verify( &[] ).unwrap().to_string(), "balanced: entries 0, net 0" );
  assert_eq!
  (
    verify( &[ Entry::new( "x", -1 ) ] ).unwrap().to_string(),
    "UNBALANCED: entries 1, net -1 minor units",
  );
}

/// The accumulator holds a total the posting type could not.
#[ test ]
fn the_accumulator_holds_a_total_the_posting_type_could_not()
{
  let log : Vec< Entry > = ( 0..4 ).map( | _ | Entry::new( "x", i64::MAX ) ).collect();

  let total = verify( &log ).unwrap().discrepancy_minor();

  assert_eq!( total, i128::from( i64::MAX ) * 4 );
  assert!( total > i128::from( u64::MAX ), "the total must exceed what 64 bits can hold" );
}

/// The log's records are plain data anyone can build.
#[ test ]
fn a_log_can_be_built_from_nothing_but_integers()
{
  let log : Vec< Entry > = [ ( "a", -5_i64 ), ( "b", 5_i64 ) ]
  .into_iter()
  .map( | ( account, amount ) | Entry::new( account, amount ) )
  .collect();

  let report : Report = verify( &log ).unwrap();
  assert!( report.is_balanced() );
}

/// `ConservationError::Overflow` carries no position — it names the
/// condition, not a place to bisect to, unlike `exact_audit`'s own
/// `AccumulatorOverflow { at_entry }`.
#[ test ]
fn the_overflow_error_names_the_representable_range()
{
  assert_eq!( ConservationError::Overflow.to_string(), "the running total left the representable range" );
}

/// `money_conserve_into` folds legs the same way `exact_add::money_add`
/// does, and is usable directly as a `try_fold` step.
#[ test ]
fn money_conserve_into_folds_legs_via_exact_add_and_is_usable_with_try_fold()
{
  let legs = [ Money::from_minor( 500 ).unwrap(), Money::from_minor( -500 ).unwrap(), Money::from_minor( 125 ).unwrap() ];
  let total = legs.iter().copied().try_fold( Money::ZERO, money_conserve_into ).unwrap();
  assert_eq!( total, Money::from_minor( 125 ).unwrap() );
}

/// `money_conserve_into` reports overflow the same way `exact_add::money_add`
/// does, rather than wrapping or panicking.
#[ test ]
fn money_conserve_into_reports_overflow_past_the_declared_ceiling()
{
  assert_eq!( money_conserve_into( Money::MAX, Money::EPSILON ), Err( ConservationError::Overflow ) );
}

/// `qty_conserve_into` folds legs the same way `exact_add::qty_add` does.
#[ test ]
fn qty_conserve_into_folds_legs_via_exact_add()
{
  let legs = [ Quantity::from_minor( 3 ).unwrap(), Quantity::from_minor( 4 ).unwrap() ];
  let total = legs.iter().copied().try_fold( Quantity::ZERO, qty_conserve_into ).unwrap();
  assert_eq!( total, Quantity::from_minor( 7 ).unwrap() );
}

/// `money_sum_assert_zero` passes when a typed slice of legs cancels
/// exactly, and reports the exact signed discrepancy otherwise.
#[ test ]
fn money_sum_assert_zero_passes_when_legs_cancel_and_reports_the_exact_discrepancy_otherwise()
{
  let balanced = [ Money::from_minor( 1_000_000 ).unwrap(), Money::from_minor( -1_000_000 ).unwrap() ];
  assert_eq!( money_sum_assert_zero( &balanced ), Ok( () ) );

  let leaky = [ Money::from_minor( 1_000_000 ).unwrap(), Money::from_minor( -999_999 ).unwrap() ];
  assert_eq!( money_sum_assert_zero( &leaky ), Err( ConservationError::NotZero { got : 1 } ) );
}

/// An empty slice of money legs vacuously sums to zero.
#[ test ]
fn money_sum_assert_zero_passes_on_an_empty_slice()
{
  assert_eq!( money_sum_assert_zero( &[] ), Ok( () ) );
}

/// `qty_sum_assert_zero` passes only when every leg is zero — a non-negative
/// kind's sum can never cancel the way signed money legs do.
#[ test ]
fn qty_sum_assert_zero_passes_only_when_every_leg_is_zero()
{
  assert_eq!( qty_sum_assert_zero( &[ Quantity::ZERO, Quantity::ZERO ] ), Ok( () ) );

  let holding = [ Quantity::ZERO, Quantity::from_minor( 3 ).unwrap() ];
  assert_eq!( qty_sum_assert_zero( &holding ), Err( ConservationError::NotZero { got : 3 } ) );
}
