//! Hard problem 12 (hot path) and feature 22's own proposed verification: a
//! documented timing comparison between this family's checked arithmetic and
//! `f64`, exercising exactly the three operations hard problem 12 names —
//! add, compare, ratio. See `docs/hard_problem/012_hot_path_performance.md`
//! and `docs/feature/022_bench_note_vs_f64.md`.
//!
//! Informational, not a performance gate: wall-clock timing is
//! environment-dependent, so the one assertion below is a loose sanity bound
//! against a catastrophic regression (an accidental heap allocation or
//! similar per-op cost), never a strict "must beat `f64`" claim. Run with
//! `cargo test --test bench_vs_f64 -- --nocapture` to see the printed report.
//!
//! The second test below extends the comparison to
//! [`primitive_fixed_point_decimal`](https://docs.rs/primitive_fixed_point_decimal),
//! the closest published analog identified in
//! `docs/research/001_exact_vs_open_source_alternatives.md` — a dev-only
//! dependency used solely by this test, never by the family's own production
//! code, so it does not touch the zero-external-dependency convention that
//! research doc already notes was a choice, not a forced hand.

use exact_arith::{ money_cmp, money_mul_ratio, ratio_new, Money };
use primitive_fixed_point_decimal::{ fpdec, ConstScaleFpdec };
use std::hint::black_box;
use std::time::{ Duration, Instant };

type PfpdMoney = ConstScaleFpdec< i64, 6 >;

const ITERS : u64 = 10_000_000;

fn time_it< F : FnMut() >( mut f : F ) -> Duration
{
  let start = Instant::now();
  for _ in 0..ITERS
  {
    f();
  }
  start.elapsed()
}

fn ns_per_op( d : Duration ) -> f64
{
  d.as_nanos() as f64 / ITERS as f64
}

#[ test ]
fn add_compare_ratio_vs_f64()
{
  let a = Money::parse( "1234.56" ).unwrap();
  let b = Money::parse( "0.01" ).unwrap();
  let fa : f64 = 1234.56;
  let fb : f64 = 0.01;

  let exact_add = time_it( || { black_box( black_box( a ).checked_add( black_box( b ) ).unwrap() ); } );
  let f64_add = time_it( || { black_box( black_box( fa ) + black_box( fb ) ); } );

  let exact_cmp = time_it( || { black_box( money_cmp( black_box( a ), black_box( b ) ) ); } );
  let f64_cmp = time_it( || { black_box( black_box( fa ).partial_cmp( &black_box( fb ) ) ); } );

  let ratio = ratio_new( 3, 4 ).unwrap();
  let exact_ratio = time_it( || { black_box( money_mul_ratio( black_box( a ), black_box( ratio ) ).unwrap() ); } );
  let f64_ratio = time_it( || { black_box( black_box( fa ) * black_box( 0.75_f64 ) ); } );

  println!( "\n=== exact vs f64, {ITERS} iterations each (ns/op) ===" );
  println!( "add    : exact={:>7.3}  f64={:>7.3}  slowdown={:.1}x", ns_per_op( exact_add ), ns_per_op( f64_add ), ns_per_op( exact_add ) / ns_per_op( f64_add ).max( 0.001 ) );
  println!( "compare: exact={:>7.3}  f64={:>7.3}  slowdown={:.1}x", ns_per_op( exact_cmp ), ns_per_op( f64_cmp ), ns_per_op( exact_cmp ) / ns_per_op( f64_cmp ).max( 0.001 ) );
  println!( "ratio  : exact={:>7.3}  f64={:>7.3}  slowdown={:.1}x", ns_per_op( exact_ratio ), ns_per_op( f64_ratio ), ns_per_op( exact_ratio ) / ns_per_op( f64_ratio ).max( 0.001 ) );

  // Loose sanity bound only (see module doc comment): catches a catastrophic
  // regression, not normal scheduling noise. +5.0 absorbs sub-nanosecond f64
  // timings where any multiplier would otherwise be meaningless.
  const MAX_SLOWDOWN : f64 = 100.0;
  assert!( ns_per_op( exact_add ) < ns_per_op( f64_add ) * MAX_SLOWDOWN + 5.0, "add: exact arithmetic regressed far beyond f64's cost" );
  assert!( ns_per_op( exact_cmp ) < ns_per_op( f64_cmp ) * MAX_SLOWDOWN + 5.0, "compare: exact arithmetic regressed far beyond f64's cost" );
  assert!( ns_per_op( exact_ratio ) < ns_per_op( f64_ratio ) * MAX_SLOWDOWN + 5.0, "ratio: exact arithmetic regressed far beyond f64's cost" );
}

#[ test ]
fn add_compare_ratio_vs_primitive_fixed_point_decimal()
{
  let a = Money::parse( "1234.56" ).unwrap();
  let b = Money::parse( "0.01" ).unwrap();
  let pa : PfpdMoney = fpdec!( 1234.56 );
  let pb : PfpdMoney = fpdec!( 0.01 );

  let exact_add = time_it( || { black_box( black_box( a ).checked_add( black_box( b ) ).unwrap() ); } );
  let pfpd_add = time_it( || { let r : PfpdMoney = black_box( pa ) + black_box( pb ); black_box( r ); } );

  let exact_cmp = time_it( || { black_box( money_cmp( black_box( a ), black_box( b ) ) ); } );
  let pfpd_cmp = time_it( || { black_box( black_box( pa ).partial_cmp( &black_box( pb ) ) ); } );

  let ratio = ratio_new( 3, 4 ).unwrap();
  let p_ratio : PfpdMoney = fpdec!( 0.75 );
  let exact_ratio = time_it( || { black_box( money_mul_ratio( black_box( a ), black_box( ratio ) ).unwrap() ); } );
  let pfpd_ratio = time_it( || { let r : PfpdMoney = black_box( pa ) * black_box( p_ratio ); black_box( r ); } );

  println!( "\n=== exact vs primitive_fixed_point_decimal (ConstScaleFpdec<i64,6>), {ITERS} iterations each (ns/op) ===" );
  println!( "add    : exact={:>7.3}  pfpd={:>7.3}  ratio={:.1}x", ns_per_op( exact_add ), ns_per_op( pfpd_add ), ns_per_op( exact_add ) / ns_per_op( pfpd_add ).max( 0.001 ) );
  println!( "compare: exact={:>7.3}  pfpd={:>7.3}  ratio={:.1}x", ns_per_op( exact_cmp ), ns_per_op( pfpd_cmp ), ns_per_op( exact_cmp ) / ns_per_op( pfpd_cmp ).max( 0.001 ) );
  println!( "ratio  : exact={:>7.3}  pfpd={:>7.3}  ratio={:.1}x", ns_per_op( exact_ratio ), ns_per_op( pfpd_ratio ), ns_per_op( exact_ratio ) / ns_per_op( pfpd_ratio ).max( 0.001 ) );

  // Informational only, same non-gating rationale as the f64 test above —
  // this is the "closest published analog" comparison the research doc's
  // own Conclusion flags as the natural next step, not a claim this family
  // should switch to it.
}
