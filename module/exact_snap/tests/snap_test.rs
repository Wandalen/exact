//! Tick/lot construction, and snapping a price or quantity onto the grid
//! under every rounding mode.

use exact_kind::{ Price, Quantity };
use exact_round::Rounding;
use exact_snap::{ Lot, SnapError, Tick, price_snap_tick, qty_snap_lot };

/// A zero-sized tick or lot is refused at construction.
#[ test ]
fn zero_sized_tick_and_lot_are_refused_at_construction()
{
  assert_eq!( Tick::new( Price::ZERO ), Err( SnapError::ZeroTick ) );
  assert_eq!( Lot::new( Quantity::ZERO ), Err( SnapError::ZeroLot ) );
}

/// Snapping a value already exactly on the grid returns it unchanged, at
/// every rounding mode.
#[ test ]
fn a_value_already_on_the_grid_is_unchanged_at_every_rounding_mode()
{
  let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
  let on_grid = Price::from_minor( 15 ).unwrap();
  for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven ]
  {
    assert_eq!( price_snap_tick( on_grid, tick, mode ).unwrap(), on_grid );
  }
}

/// `Down` snaps toward the nearer-or-equal grid point below.
#[ test ]
fn down_snaps_to_the_grid_point_at_or_below()
{
  let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
  let between = Price::from_minor( 17 ).unwrap(); // between 15 and 20
  assert_eq!( price_snap_tick( between, tick, Rounding::Down ).unwrap().minor(), 15 );
}

/// `Up` snaps toward the nearer-or-equal grid point above.
#[ test ]
fn up_snaps_to_the_grid_point_at_or_above()
{
  let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
  let between = Price::from_minor( 17 ).unwrap();
  assert_eq!( price_snap_tick( between, tick, Rounding::Up ).unwrap().minor(), 20 );
}

/// `HalfEven` snaps an exact tie to the even multiple.
#[ test ]
fn half_even_snaps_an_exact_tie_to_the_even_multiple()
{
  let tick = Tick::new( Price::from_minor( 10 ).unwrap() ).unwrap();
  let tie = Price::from_minor( 15 ).unwrap(); // exactly between 10 (1x, odd) and 20 (2x, even)
  assert_eq!( price_snap_tick( tie, tick, Rounding::HalfEven ).unwrap().minor(), 20 );
}

/// Quantity snapping onto a lot behaves the same as price snapping onto a tick.
#[ test ]
fn qty_snap_lot_behaves_the_same_as_price_snap_tick()
{
  let lot = Lot::new( Quantity::from_minor( 5 ).unwrap() ).unwrap();
  let between = Quantity::from_minor( 17 ).unwrap();
  assert_eq!( qty_snap_lot( between, lot, Rounding::Down ).unwrap().minor(), 15 );
  assert_eq!( qty_snap_lot( between, lot, Rounding::Up ).unwrap().minor(), 20 );
}

/// Snapping never produces a negative quantity — every input and every grid
/// point here is already non-negative.
#[ test ]
fn qty_snap_lot_never_produces_a_negative_result()
{
  let lot = Lot::new( Quantity::from_minor( 7 ).unwrap() ).unwrap();
  let small = Quantity::from_minor( 2 ).unwrap(); // less than one lot
  assert_eq!( qty_snap_lot( small, lot, Rounding::Down ).unwrap(), Quantity::ZERO );
  assert_eq!( qty_snap_lot( small, lot, Rounding::Up ).unwrap().minor(), 7 );
}

/// Snapping up from just below the ceiling, on a tick the ceiling isn't an
/// exact multiple of, pushes the result past the declared range —
/// `SnapError::Overflow`, distinct from the `ZeroTick`/`ZeroLot` cases above.
/// `Price::MAX` is `9_000_000_000_000_000` (`9e9` whole units at scale 6),
/// whose only prime factors are 2, 3, and 5, so tick `7` never divides it
/// evenly and rounding up from 4 short of the ceiling overshoots to
/// `Price::MAX + 2`.
#[ test ]
fn price_snap_tick_reports_overflow_rounding_up_past_the_ceiling()
{
  let near_ceiling = Price::from_minor( Price::MAX.minor() - 4 ).unwrap();
  let tick = Tick::new( Price::from_minor( 7 ).unwrap() ).unwrap();
  assert_eq!( price_snap_tick( near_ceiling, tick, Rounding::Up ), Err( SnapError::Overflow ) );
}
