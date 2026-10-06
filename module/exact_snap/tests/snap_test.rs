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

/// `HalfEven` off a tie snaps to the nearer grid point, whichever side it is.
#[ test ]
fn half_even_off_a_tie_snaps_to_the_nearer_grid_point()
{
  let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
  let nearer_below = Price::from_minor( 17 ).unwrap(); // 2 from 15, 3 from 20
  let nearer_above = Price::from_minor( 18 ).unwrap(); // 3 from 15, 2 from 20
  assert_eq!( price_snap_tick( nearer_below, tick, Rounding::HalfEven ).unwrap().minor(), 15 );
  assert_eq!( price_snap_tick( nearer_above, tick, Rounding::HalfEven ).unwrap().minor(), 20 );
}

/// A negative price snaps the same way: `Down` toward negative infinity, `Up`
/// toward positive infinity — `-17` lies between `-20` and `-15`.
#[ test ]
fn a_negative_price_snaps_down_to_the_lower_grid_point_and_up_to_the_higher()
{
  let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
  let negative = Price::from_minor( -17 ).unwrap();
  assert_eq!( price_snap_tick( negative, tick, Rounding::Down ).unwrap().minor(), -20 );
  assert_eq!( price_snap_tick( negative, tick, Rounding::Up ).unwrap().minor(), -15 );
}

/// A negative tick marks the same grid as its positive counterpart, so it
/// snaps identically.
///
/// Root Cause: `price_snap_tick` divided the price by the signed tick, so
/// for a tick of -5 the tick count was rounded on a reversed axis and then
/// multiplied back by the negative spacing — `Down` snapped up and `Up` down.
///
/// Why Not Caught: every snap test used a positive tick; `Tick::new` accepts
/// a negative one, but nothing exercised it.
///
/// Fix Applied: the price is divided by the tick's magnitude
/// (`tick.0.minor().abs()`), so the sign of the tick no longer matters.
///
/// Prevention: this test compares a tick of -5 with a tick of 5 on prices
/// above, below and on the grid under every mode; it fails on the old code.
///
/// Pitfall: `Down`/`Up` round a quotient toward -∞/+∞; multiplied back by a
/// negative divisor, that direction reverses for the value itself.
#[ test ]
fn a_negative_tick_snaps_exactly_like_its_positive_counterpart()
{
  let positive = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
  let negative = Tick::new( Price::from_minor( -5 ).unwrap() ).unwrap();
  for v in [ 17, -17, 15, 0 ]
  {
    let price = Price::from_minor( v ).unwrap();
    for mode in [ Rounding::Down, Rounding::Up, Rounding::HalfEven ]
    {
      assert_eq!( price_snap_tick( price, negative, mode ), price_snap_tick( price, positive, mode ) );
    }
  }
  let between = Price::from_minor( 17 ).unwrap();
  assert_eq!( price_snap_tick( between, negative, Rounding::Down ).unwrap().minor(), 15 );
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

/// A tick or lot hands back exactly the size it was built from — a negative
/// tick included, which `Tick::new` accepts.
#[ test ]
fn tick_and_lot_return_the_size_they_were_built_from()
{
  let size = Price::from_minor( -5 ).unwrap();
  assert_eq!( Tick::new( size ).unwrap().price(), size );
  let lot = Quantity::from_minor( 3 ).unwrap();
  assert_eq!( Lot::new( lot ).unwrap().qty(), lot );
}

/// `HalfEven` on a lot: off a tie to the nearer point, on a tie to the even
/// multiple — `15` is halfway between `10` (1×, odd) and `20` (2×, even).
#[ test ]
fn qty_snap_lot_half_even_takes_the_nearer_point_and_breaks_a_tie_to_even()
{
  let five = Lot::new( Quantity::from_minor( 5 ).unwrap() ).unwrap();
  let seventeen = Quantity::from_minor( 17 ).unwrap();
  assert_eq!( qty_snap_lot( seventeen, five, Rounding::HalfEven ).unwrap().minor(), 15 );
  let ten = Lot::new( Quantity::from_minor( 10 ).unwrap() ).unwrap();
  let fifteen = Quantity::from_minor( 15 ).unwrap();
  assert_eq!( qty_snap_lot( fifteen, ten, Rounding::HalfEven ).unwrap().minor(), 20 );
}

/// Snapping a quantity up past the ceiling is refused, as for a price.
#[ test ]
fn qty_snap_lot_reports_overflow_rounding_up_past_the_ceiling()
{
  let near_ceiling = Quantity::from_minor( Quantity::MAX.minor() - 4 ).unwrap();
  let lot = Lot::new( Quantity::from_minor( 7 ).unwrap() ).unwrap();
  assert_eq!( qty_snap_lot( near_ceiling, lot, Rounding::Up ), Err( SnapError::Overflow ) );
}
