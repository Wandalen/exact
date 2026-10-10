//! Ported from `exact_arithmetic/tests/facade_test.rs`'s T10/C4 coverage,
//! updated for the 14-leaf surface: two opposite failures are possible for a
//! re-export crate — *incomplete* (a consumer has to reach past it to a tier
//! crate) or *too much* (it grows a helper of its own) — and this file
//! covers both, in-crate, so the failure is reported next to the cause.

use exact_arith::
{
  money_dust_split, money_from_wire, money_to_wire, price_mul_qty, round_div, verify, ConservationError,
  DustTo, Entry, KindError, Money, Price, Quantity, Report, Rounding, Sign, sign_of, CEILING_MINOR_UNITS,
  CEILING_WHOLE_UNITS, MONEY_SCALE, pow10,
};

/// T10 — a whole settlement runs with `exact_arith` as the only import.
///
/// Deliberately written as one continuous piece of work rather than as a
/// list of name references: a facade is complete when something can be
/// *done* through it, not when its names resolve.
#[ test ]
fn a_settlement_runs_end_to_end_through_the_facade_alone()
{
  let price = Price::parse( "1.25" ).unwrap();
  let filled = Quantity::from_int( 4 ).unwrap();

  let notional = price_mul_qty( price, filled, Rounding::HalfEven ).unwrap();
  assert_eq!( notional, Money::parse( "5" ).unwrap() );

  let log = [ Entry::new( "buyer", "cash", -notional.minor() ), Entry::new( "seller", "cash", notional.minor() ) ];
  let report : Report< &str > = verify( &log ).unwrap();
  assert!( report.is_balanced() );

  let remaining = filled.checked_sub( Quantity::from_int( 4 ).unwrap() ).unwrap();
  assert_eq!( remaining, Quantity::ZERO );

  // Touch one item from each of the four newer tiers, so completeness is
  // checked against the full 14-crate surface, not only the 3 crates the
  // original `exact_arithmetic` facade covered.
  let shares = money_dust_split( notional, 5, Rounding::Down, DustTo::First ).unwrap();
  assert_eq!( shares.iter().copied().try_fold( Money::ZERO, | a, b | a.checked_add( b ) ).unwrap(), notional );

  let wire = money_to_wire( notional );
  assert_eq!( money_from_wire( wire ).unwrap(), notional );
}

/// T10 — a representative re-exported name from each tier resolves to the
/// tier crate's own item, checked by use rather than by existence: a
/// re-export that resolved to the wrong item would still compile against a
/// bare mention.
#[ test ]
fn every_re_exported_name_resolves_to_the_tier_crate_item()
{
  assert_eq!( MONEY_SCALE, 6 );
  assert_eq!( pow10( MONEY_SCALE ), 1_000_000 );
  assert_eq!( CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS * pow10( MONEY_SCALE ) );

  assert_eq!( round_div( 7, 2, Rounding::Down ).unwrap(), 3 );
  assert_eq!( sign_of( -5 ), Sign::Neg );

  assert!( matches!( Money::parse( "0.1234567" ), Err( KindError::ExcessPrecision { .. } ) ) );
  assert!( matches!( Quantity::from_int( -1 ), Err( KindError::Negative { .. } ) ) );
  assert_eq!( ConservationError::Overflow.to_string(), "the running total left the representable range" );
}

/// C4 — the facade declares nothing of its own.
///
/// Read from the source that is compiled into this test, so it cannot drift
/// from the crate that is actually built. Every keyword below would
/// introduce an item into a crate whose job is to have none; `use` is the
/// only thing left.
#[ test ]
fn the_facade_source_is_re_exports_and_documentation_only()
{
  const SOURCE : &str = include_str!( "../src/lib.rs" );

  let code : Vec< &str > = SOURCE
  .lines()
  .map( str::trim )
  .filter( | line | !line.is_empty() && !line.starts_with( "//" ) )
  .collect();

  for keyword in [ "fn ", "struct ", "enum ", "trait ", "impl ", "const ", "static ", "type ", "macro_rules" ]
  {
    let offenders : Vec< &&str > = code.iter().filter( | line | line.contains( keyword ) ).collect();
    assert!
    (
      offenders.is_empty(),
      "exact_arith must re-export only, but `{keyword}` appears in {offenders:?}",
    );
  }

  assert!( code.iter().any( | line | line.starts_with( "pub use" ) ), "a facade with no re-exports is not one" );
}

/// The backing width is declared once across the whole family.
///
/// The range budget's own check, run as a test rather than left as a command
/// in a document. A second declaration is how the family acquires two
/// widths and discovers it at the seam between two crates that each
/// believed the other agreed with them.
#[ test ]
fn the_backing_width_is_declared_in_exactly_one_place()
{
  let sources =
  [
    ( "exact_minor", include_str!( "../../exact_minor/src/lib.rs" ) ),
    ( "exact_kind", include_str!( "../../exact_kind/src/lib.rs" ) ),
    ( "exact_arith", include_str!( "../src/lib.rs" ) ),
  ];

  let declarations : Vec< &str > = sources
  .iter()
  .filter( | ( _, source ) | source.contains( "pub type Backing" ) )
  .map( | ( name, _ ) | *name )
  .collect();

  assert_eq!( declarations, [ "exact_minor" ] );
}
