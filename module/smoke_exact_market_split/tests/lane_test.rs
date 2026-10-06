//! The lane's six steps, driven without the binary.
//!
//! Ported from `smoke_exact_arithmetic/tests/lane_test.rs` — the first four
//! tests carry forward unchanged except for the import path; the fifth is
//! new, covering step 5's market split; the last two cover step 6's scenes.
//!
//! The lane is its own test — every claim it prints is an assertion — but
//! that only holds while something runs it. No test suite can reach a bare
//! `src/main.rs` to raise its coverage, so the file carrying every assertion
//! in the slice was also the file nothing measured; `tests/lane_test.rs`
//! drives it instead.
//!
//! [`run`]: smoke_exact_market_split::run

use exact_arith::{ Decimal, Money, Price, Quantity, verify };
use smoke_exact_market_split::
{
  REPEATS, checksum, exact_tenths, float_tenths, golden, ledger, market_split, run,
};

/// Ten tenths added exactly are exactly one.
///
/// The value the whole family exists to get right, asserted against the
/// parsed literal rather than against a constructed `Money`, so a parser
/// that drifted would not quietly supply both sides of the comparison.
#[ test ]
fn the_exact_arm_sums_to_exactly_one()
{
  assert_eq!
  (
    exact_tenths(),
    Money::parse( "1.0" ).expect( "1.0 parses" ),
    "0.1 added {REPEATS} times through the exact path did not come to one"
  );
}

/// The control arm still disagrees.
///
/// This is the assertion that keeps every other one meaningful: the lane's
/// verdict rests on the exact path and the `f64` path reaching different
/// answers, so an `f64` sum that happened to land on 1.0 would leave the
/// lane printing a pass it had not earned.
#[ test ]
fn the_control_arm_is_still_wrong()
{
  #[ allow( clippy::float_cmp ) ] // Being unequal to 1.0 is the whole assertion.
  {
    assert!
    (
      float_tenths() != 1.0_f64,
      "the f64 control arm agreed with the exact path, so the lane no longer \
       discriminates between exact and inexact arithmetic"
    );
  }
}

/// The audit names a leak's signed magnitude, not merely its existence.
#[ test ]
fn the_audit_separates_a_balanced_log_from_a_one_unit_leak()
{
  let amount = exact_tenths();

  let balanced = verify( &ledger( amount, 0 ) ).expect( "a two-posting log cannot overflow i128" );
  assert!( balanced.is_balanced(), "a log with matching postings was reported as leaking" );

  let leaky = verify( &ledger( amount, 1 ) ).expect( "a two-posting log cannot overflow i128" );
  assert!( !leaky.is_balanced(), "a one-unit leak went undetected" );
  assert_eq!( leaky.discrepancy_minor(), -1, "the leak was detected but its signed magnitude was not" );
}

/// The lane runs end to end and every step's assertion holds.
#[ test ]
fn the_lane_runs_end_to_end()
{
  run();
}

/// `ledger` refuses a `leak_minor` that would overflow the seller posting
/// instead of silently wrapping it.
///
/// Root Cause: `ledger` computed `amount.minor() - leak_minor` with a bare
/// `-` on two `i64`s. `amount.minor()` is bounded well inside `i64` by the
/// exact decimal type's own ceiling, but `leak_minor` is a public,
/// unvalidated `i64` parameter with no such bound — passing `leak_minor`
/// near `i64::MIN` drives the subtraction past `i64::MAX`: panicking with no
/// diagnostic in a debug build, and in release silently wrapping to a
/// near-`i64::MIN` "seller" posting that the auditor would report as an
/// ordinary, if enormous, discrepancy rather than the corrupted computation
/// it was.
///
/// Why Not Caught: every existing call to `ledger` — inside `run` and in
/// `the_audit_separates_a_balanced_log_from_a_one_unit_leak` above — passes
/// `leak_minor` of `0` or `1`. No test drove `leak_minor` anywhere near a
/// value that could interact with `i64`'s range.
///
/// Fix Applied: `ledger` now computes the seller posting via
/// `amount.minor().checked_sub( leak_minor ).expect( .. )`, matching every
/// other arithmetic operation in this crate family, which already routes
/// through a `checked_*` method rather than a bare operator. `expect` rather
/// than a `Result` return keeps this lane's own established idiom — see
/// `exact_tenths`'s identical `checked_add( .. ).expect( .. )` — since every
/// real caller supplies a small, curated demo constant and a violation here
/// is this lane's own bug, not a condition its caller must handle.
///
/// Prevention: a `leak_minor` that cannot be subtracted from `amount.minor()`
/// without overflowing now panics loudly and immediately, in both debug and
/// release, instead of wrapping to a nonsense minor-unit figure that the
/// auditor would audit as if it were real.
///
/// Pitfall: a value bounded by one type's own ceiling (`Money`/`Backing`)
/// does not bound an arithmetic expression that mixes it with an
/// unconstrained plain integer parameter — each operand needs its own check,
/// not just the one that happens to come from a validated type.
#[ test ]
#[ should_panic( expected = "leak_minor is a small demo constant well within i64 range" ) ]
fn a_leak_minor_that_would_overflow_the_seller_posting_panics_instead_of_wrapping()
{
  let amount = Money::parse( "0.0001" ).expect( "0.0001 is representable at scale 6" );
  let _ = ledger( amount, i64::MIN );
}

/// A market split's shares recombine to exactly the original fill, dust
/// included — the new step this lane adds over `smoke_exact_arithmetic`.
#[ test ]
fn the_market_split_recombines_to_the_original_fill()
{
  let fill = Money::parse( "100.000001" ).expect( "parses" );
  let shares = market_split( fill, 3 );
  assert_eq!( shares.len(), 3 );

  let recombined = shares.iter().copied().try_fold( Money::ZERO, | a, b | a.checked_add( b ) )
  .expect( "three small shares recombine within range" );
  assert_eq!( recombined, fill, "a market split must conserve the original fill exactly" );

  // The dust (100_000_001 minor units does not divide evenly by 3) lands
  // entirely on the first share — the other two are exactly equal.
  assert_eq!( shares[ 1 ], shares[ 2 ] );
  assert!( shares[ 0 ] > shares[ 1 ], "the first share must absorb the remainder" );
}

/// Step 6 — every scene of `docs/scene/` lands on its golden value. Scenes
/// 003 and 009 run on `Money`, whose scale is 6, so their values are the
/// scale-6 spelling of the proposed `3.33`/`0.01` and `1000`.
#[ test ]
fn the_scenes_land_on_their_golden_values()
{
  let g = golden();
  assert_eq!( g.sum, Decimal::< 2 >::ZERO, "scene 002: 10.00 + 3.33 - 13.33" );
  assert_eq!( g.parts, vec![ Money::from_minor( 3_333_333 ).unwrap(); 3 ], "scene 003: the parts" );
  assert_eq!( g.dust_minor, 1, "scene 003: the dust" );
  assert_eq!( g.tick, Price::parse( "1.25" ).unwrap(), "scene 004" );
  assert_eq!( g.lot, Quantity::from_int( 9 ).unwrap(), "scene 005" );
  assert!( g.extra, "scene 006" );
  assert!( g.overflow, "scene 008" );
  assert_eq!( g.wire_minor, 10_000_000, "scene 009" );
}

/// Scene 010 — two runs produce the same checksum, and the checksum is not a
/// constant: one minor unit of difference changes it.
#[ test ]
fn the_checksum_is_stable_across_calls_and_changes_with_any_value()
{
  let g = golden();
  assert_eq!( checksum( &g ), checksum( &golden() ) );

  let mut changed = g.clone();
  changed.dust_minor += 1;
  assert_ne!( checksum( &g ), checksum( &changed ) );
}
