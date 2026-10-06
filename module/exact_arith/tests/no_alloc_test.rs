//! Rendering into a caller's buffer and splitting into a caller's slice
//! allocate nothing — checked with `assert_no_alloc`'s per-thread counting
//! allocator, so an allocation that comes back fails here even though the
//! output would still be right.
//!
//! The allocator's `unsafe impl GlobalAlloc` lives in the `assert_no_alloc`
//! dev-dependency; this file only selects it with `#[ global_allocator ]`,
//! which is safe code, so the workspace's `unsafe-code = "deny"` holds here
//! too. The selection applies to this test binary only.

use assert_no_alloc::{ assert_no_alloc, reset_violation_count, violation_count, AllocDisabler };
use exact_arith::
{
  fmt_into, money_dust_split_into, qty_dust_split_into, DustTo, Money, Price, Quantity, Rounding,
};

#[ global_allocator ]
static ALLOCATOR : AllocDisabler = AllocDisabler;

/// How many allocator calls the current thread makes while `f` runs.
fn allocations_during( f : impl FnOnce() ) -> u32
{
  reset_violation_count();
  assert_no_alloc( f );
  violation_count()
}

/// The counter itself works: building a `Vec` registers, so a broken
/// counter cannot make the two tests below pass.
#[ test ]
fn the_counter_sees_an_allocation()
{
  let count = allocations_during( || { std::hint::black_box( Vec::< u8 >::with_capacity( 8 ) ); } );
  assert!( count > 0 );
}

/// `Display` for every kind, rendered through `fmt_into` into a stack
/// buffer, allocates nothing — negative, fractional and whole values alike.
/// Guards the fix recorded at `Fix(exact_kind_display_allocated_per_render)`.
#[ test ]
fn rendering_into_a_stack_buffer_does_not_allocate()
{
  let money = [ "-1234.5", "0.000001", "42" ].map( | text | Money::parse( text ).unwrap() );
  let qty = Quantity::parse( "2.75" ).unwrap();
  let price = Price::parse( "-0.1" ).unwrap();
  let mut buf = [ 0_u8; 64 ];
  let count = allocations_during( ||
  {
    for v in money
    {
      fmt_into( v, &mut buf ).unwrap();
    }
    fmt_into( qty, &mut buf ).unwrap();
    fmt_into( price, &mut buf ).unwrap();
  } );
  assert_eq!( count, 0 );
  let len = fmt_into( money[ 0 ], &mut buf ).unwrap();
  assert_eq!( &buf[ .. len ], b"-1234.5" );
}

/// Both `*_dust_split_into` functions fill the caller's slice without
/// allocating. Guards the fix recorded at `Fix(exact_dust_split_into_allocated)`.
#[ test ]
fn both_dust_split_into_functions_do_not_allocate()
{
  let total = Money::from_minor( 11 ).unwrap();
  let held = Quantity::from_minor( 11 ).unwrap();
  let mut money_out = [ Money::ZERO; 4 ];
  let mut qty_out = [ Quantity::ZERO; 4 ];
  let count = allocations_during( ||
  {
    money_dust_split_into( total, Rounding::Down, DustTo::First, &mut money_out ).unwrap();
    qty_dust_split_into( held, Rounding::Down, DustTo::First, &mut qty_out ).unwrap();
  } );
  assert_eq!( count, 0 );
  assert_eq!( money_out[ 0 ].minor(), 5 );
  assert_eq!( qty_out[ 1 ].minor(), 2 );
}
