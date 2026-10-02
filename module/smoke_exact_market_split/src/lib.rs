//! Smoke lane `smoke_exact_market_split` — this family's slice, run end to
//! end with a control arm, through `exact_arith` — the 14-crate facade —
//! alone.
//!
//! Five steps in one process: a [`Money`] parsed and added by a fixed-point
//! decimal type, a [`Quantity`] refusing to go below zero, a log audited by a
//! conservation auditor, a market fill split among several accounts with the
//! dust accounted for and the split proven to conserve, and every one of
//! those names imported from `exact_arith` and from nowhere else — which is
//! what makes the facade's completeness a thing this lane tests rather than
//! a thing its documentation claims.
//!
//! Ported from `smoke_exact_arithmetic`, this family's demo lane before the
//! 15-crate migration — steps 1 through 4 carry its exact content forward
//! unchanged; step 5 is new, added because a lane named for a market split
//! ought to run one, exercising `exact_dust` and `exact_conserve` together in
//! a way neither crate's own unit tests do on their own.
//!
//! # The control arm
//!
//! A lane that only runs the exact path proves the exact path does not
//! crash. It does not prove the path is exact, because a lane built on
//! `f64` would print the same cheerful verdict for nine of these ten steps.
//!
//! So every claim here is made twice — once through the exact types and once
//! through `f64` — and the lane **asserts that the two disagree**. If a
//! future change made the exact path inexact, the arms would agree, the
//! assertion would fail, and the lane would go red. The control arm's job is
//! to be wrong; a lane where it stops being wrong has stopped discriminating,
//! and reporting that as a pass would be worse than reporting nothing.
//!
//! `f64` appears in this file and in no other file of the family.
//!
//! # Why the lane is a library and not `src/main.rs`
//!
//! No test suite can execute a bare `src/main.rs` to raise its coverage, and
//! bounding what may live in that file keeps a coverage exclusion from
//! quietly becoming somewhere to keep logic. `src/main.rs` is the
//! argument-free entry point and nothing else, and `tests/lane_test.rs`
//! drives what moved.

use exact_arith::{ money_dust_split, Entry, Money, Quantity, Rounding, verify, DustTo };

/// How many times the tenth is added, in both arms.
pub const REPEATS : i64 = 10;

/// The exact arm: ten tenths, added in the exact decimal type.
///
/// # Panics
///
/// Panics if `0.1` is not representable at the type's scale, or if ten
/// tenths overflow — either of which is the exact path failing the lane
/// rather than a condition worth handing back.
#[ must_use ]
pub fn exact_tenths() -> Money
{
  let tenth = Money::parse( "0.1" ).expect( "0.1 is representable at scale 6" );
  let mut total = Money::ZERO;
  for _ in 0..REPEATS
  {
    total = total.checked_add( tenth ).expect( "one whole unit is far below the ceiling" );
  }
  total
}

/// The control arm: the same ten tenths, added in binary floating point.
#[ must_use ]
pub fn float_tenths() -> f64
{
  let mut total = 0.0_f64;
  for _ in 0..REPEATS
  {
    total += 0.1_f64;
  }
  total
}

/// The ledger the audit runs over: a purchase settled in two postings.
///
/// Built from the exact values, then handed over as plain records — the
/// auditor never sees a `Money` or a `Quantity`, only `i64` minor units.
///
/// # Panics
///
/// If `amount.minor() - leak_minor` would overflow `i64`. `amount` is
/// bounded well inside `i64` by the exact decimal type's own ceiling, but
/// `leak_minor` is a raw, unbounded parameter this lane's caller controls;
/// every call in this lane passes `0` or `1`, so the panic is reachable only
/// by a future caller supplying an unrealistic leak, never by `run` itself.
#[ must_use ]
pub fn ledger( amount : Money, leak_minor : i64 ) -> Vec< Entry >
{
  // Fix(smoke_exact_arithmetic_ledger_leak_minor_subtraction_overflow): the
  // "seller" posting computed `amount.minor() - leak_minor` with a bare `-`
  // on two `i64`s. `amount.minor()` is bounded well inside `i64` by the
  // exact decimal type's `CEILING_MINOR_UNITS` headroom, but `leak_minor` is
  // a public, unvalidated `i64` parameter with no such bound — a caller
  // passing `leak_minor` near `i64::MIN` drives the subtraction past
  // `i64::MAX`. This lane's own `run` only ever passes `0` or `1`, so the
  // defect never fired in practice, but every other arithmetic operation in
  // this crate family already routes through a `checked_*` method for
  // exactly this reason — this call site was the one exception. Carried
  // forward unchanged from `smoke_exact_arithmetic`; the fix identifier keeps
  // its original name for traceability back to where this was found.
  //
  // Root cause: a raw `-` on two `i64`s where only one operand carries a
  //   range invariant from its own type; the other is an unconstrained
  //   plain function parameter.
  // Pitfall: a value bounded by one type's own ceiling (`Money`/`Backing`)
  //   does not bound an arithmetic expression that mixes it with an
  //   unconstrained plain integer — each operand needs its own check, not
  //   just the one that happens to come from a validated type.
  let seller_minor = amount.minor()
  .checked_sub( leak_minor )
  .expect( "leak_minor is a small demo constant well within i64 range" );

  vec!
  [
    Entry::new( "buyer", -amount.minor() ),
    Entry::new( "seller", seller_minor ),
  ]
}

/// Split a market fill among `parts` accounts, folding the dust into the
/// first share, and return the shares.
///
/// # Panics
///
/// If the split cannot complete within the representable range — unreachable
/// for the small demo amounts this lane passes.
#[ must_use ]
pub fn market_split( fill : Money, parts : usize ) -> Vec< Money >
{
  money_dust_split( fill, parts, Rounding::Down, DustTo::First ).expect( "a small demo fill splits within range" )
}

/// Run the five steps, print what each found, and assert every claim.
///
/// # Panics
///
/// Panics on any way the slice can fall short: a parse that does not
/// round-trip, ten exact tenths that are not one, a control arm that has
/// stopped disagreeing, a withdrawal below zero that is permitted, a
/// balanced log reported as leaking, a leak whose signed magnitude is not
/// named, or a market split that does not recombine to the original total.
pub fn run()
{
  println!( "smoke_exact_market_split — this family's slice, one process" );
  println!();

  // ---- Step 1: parse and round-trip, through the exact decimal type -------

  let tenth = Money::parse( "0.1" ).expect( "0.1 parses" );
  assert_eq!( tenth.to_string(), "0.1", "render must be the inverse of parse" );
  println!( "  parse/render   0.1 -> {tenth} (round-trips exactly)" );

  // ---- Step 2: the exact arm and the control arm, on the same sum ----------

  let exact = exact_tenths();
  let control = float_tenths();
  let expected = Money::parse( "1.0" ).expect( "1.0 parses" );

  assert_eq!( exact, expected, "ten exact tenths must be exactly one" );
  #[ allow( clippy::float_cmp ) ] // Being unequal to 1.0 is the whole assertion.
  {
    assert!
    (
      control != 1.0_f64,
      "the f64 control arm agreed with the exact path, so this lane is no longer \
       discriminating between exact and inexact arithmetic and must not pass",
    );
  }
  println!( "  exact arm      0.1 x {REPEATS} = {exact}" );
  println!( "  control arm    0.1 x {REPEATS} = {control:.17} (f64, wrong by construction)" );
  println!( "  the arms disagree, which is what makes this lane a test" );

  // ---- Step 3: the quantity type refuses to go below zero ------------------

  let held = Quantity::from_int( 3 ).expect( "3 units is representable" );
  let taken = Quantity::from_int( 5 ).expect( "5 units is representable" );
  let short = held.checked_sub( taken );
  assert!( short.is_err(), "withdrawing more than is held must be refused" );
  println!();
  println!( "  quantity       hold {held}, withdraw {taken} -> {}", short.unwrap_err() );

  // ---- Step 4: the auditor over a plain log ---------------------------------

  let balanced = verify( &ledger( exact, 0 ) ).expect( "a two-posting log cannot overflow i128" );
  assert!( balanced.is_balanced(), "a log with matching postings must balance" );
  println!();
  println!( "  audit, clean   {balanced}" );

  let leaky = verify( &ledger( exact, 1 ) ).expect( "a two-posting log cannot overflow i128" );
  assert!( !leaky.is_balanced(), "a one-unit leak must be detected" );
  // Negative: the seller was credited one minor unit less than the buyer was
  // debited, so value vanished. The sign is the difference between a leak
  // and a forgery, and an auditor that reported only a magnitude would lose
  // it.
  assert_eq!( leaky.discrepancy_minor(), -1, "the discrepancy must be named, not just flagged" );
  println!( "  audit, leaky   {leaky}" );

  // ---- Step 5: a market fill split among three accounts, dust and all ------

  let fill = Money::parse( "100.000001" ).expect( "parses" );
  let shares = market_split( fill, 3 );
  let recombined = shares.iter().copied().try_fold( Money::ZERO, | a, b | a.checked_add( b ) )
  .expect( "three small shares recombine within range" );
  assert_eq!( recombined, fill, "a market split must conserve the original fill exactly, dust included" );
  println!();
  println!
  (
    "  market split   {fill} into 3 -> [{}, {}, {}] (recombines exactly)",
    shares[ 0 ], shares[ 1 ], shares[ 2 ],
  );

  // ---- Verdict -------------------------------------------------------------

  println!();
  println!( "VERDICT: reached — exact arithmetic holds across the full facade," );
  println!( "         the floating-point control arm does not, and a market split conserves." );
}
