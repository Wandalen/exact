//! Snapping a price or a quantity to the nearest point on a grid — a tick
//! size for price, a lot size for quantity.
//!
//! Tier 2, depending on `exact_kind` for the conserved value types and
//! `exact_round` for [`exact_round::round_div`], the sign-handling,
//! tie-breaking division this crate's snap is built from — shared with
//! `exact_ratio` rather than duplicated here.
//!
//! Net-new: no real precedent exists for either `Tick`, `Lot`, or snapping
//! itself.
//!
//! # Examples
//!
//! ```
//! use exact_kind::Price;
//! use exact_round::Rounding;
//! use exact_snap::{ Tick, price_snap_tick };
//!
//! let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
//! let between = Price::from_minor( 17 ).unwrap();
//! assert_eq!( price_snap_tick( between, tick, Rounding::Down ).unwrap().minor(), 15 );
//! ```

use exact_kind::{ Price, Quantity };
use exact_round::Rounding;

/// Why a tick/lot could not be constructed, or a snap could not complete.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum SnapError
{
  /// A zero-sized tick was supplied.
  ZeroTick,
  /// A zero-sized lot was supplied.
  ZeroLot,
  /// The snapped result left the representable or declared range.
  Overflow,
  /// The value is not on the grid and [`Rounding::Exact`] was asked.
  OffGrid,
}

impl core::fmt::Display for SnapError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::ZeroTick => write!( f, "a zero-sized tick was supplied" ),
      Self::ZeroLot => write!( f, "a zero-sized lot was supplied" ),
      Self::Overflow => write!( f, "the snapped result left the representable or declared range" ),
      Self::OffGrid => write!( f, "the value is not on the grid and Rounding::Exact was requested" ),
    }
  }
}

impl core::error::Error for SnapError {}

fn round_error_to_snap_error( e : exact_round::RoundError, zero : SnapError ) -> SnapError
{
  match e
  {
    // Unreachable through this crate's own public API: `Tick::new` and
    // `Lot::new` already refuse a zero-sized grid, so the division this
    // crate drives through `round_div` never sees a zero divisor. Mapped
    // defensively rather than asserted, the same defensive-but-unreachable
    // pattern `exact_kind::Decimal::checked_neg` already uses.
    exact_round::RoundError::DivZero => zero,
    exact_round::RoundError::Overflow => SnapError::Overflow,
    exact_round::RoundError::Inexact => SnapError::OffGrid,
  }
}

/// A price grid's spacing — the smallest meaningful price increment.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Tick( Price );

impl Tick
{
  /// Build a tick size, refusing zero. A negative price is kept as its
  /// magnitude: a tick of -5 marks the same grid as a tick of 5.
  ///
  /// # Errors
  ///
  /// [`SnapError::ZeroTick`] when `price` is exactly zero.
  pub const fn new( price : Price ) -> Result< Self, SnapError >
  {
    if price.minor() == 0
    {
      return Err( SnapError::ZeroTick );
    }
    // The price range is symmetric about zero, so the magnitude always fits.
    match Price::from_minor( price.minor().abs() )
    {
      Ok( size ) => Ok( Self( size ) ),
      Err( _ ) => Err( SnapError::Overflow ),
    }
  }

  /// The tick size as a price.
  #[ must_use ]
  pub const fn price( self ) -> Price
  {
    self.0
  }
}

/// A quantity grid's spacing — the smallest meaningful quantity increment.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Lot( Quantity );

impl Lot
{
  /// Build a lot size, refusing zero.
  ///
  /// # Errors
  ///
  /// [`SnapError::ZeroLot`] when `qty` is exactly zero.
  pub const fn new( qty : Quantity ) -> Result< Self, SnapError >
  {
    if qty.minor() == 0
    {
      return Err( SnapError::ZeroLot );
    }
    Ok( Self( qty ) )
  }

  /// The lot size as a quantity.
  #[ must_use ]
  pub const fn qty( self ) -> Quantity
  {
    self.0
  }
}

/// Snap a price to the nearest multiple of `tick`, per `rounding`.
///
/// # Errors
///
/// [`SnapError::Overflow`] when the snapped result leaves the representable
/// or declared range. [`SnapError::OffGrid`] when `rounding` is
/// [`Rounding::Exact`] and the price is not already on the grid.
pub fn price_snap_tick( price : Price, tick : Tick, rounding : Rounding ) -> Result< Price, SnapError >
{
  // Fix(exact_snap_negative_tick_reversed_rounding): the price used to be
  // divided by the signed tick, so for a tick of -5 the count was rounded on
  // a reversed axis and multiplied back — `Down` snapped up and `Up` down.
  // A tick of -5 marks the same grid as a tick of 5; `Tick::new` stores its
  // magnitude, so the spacing is positive and `Down` means at or below.
  //
  // Root cause: rounding a quotient by a negative divisor flips its direction.
  // Pitfall: `Down`/`Up` round the quotient toward -∞/+∞; multiplied back by
  //   a negative spacing, that direction reverses for the value itself.
  let spacing = tick.0.minor();
  let q = exact_round::round_div( price.minor(), spacing, rounding )
  .map_err( | e | round_error_to_snap_error( e, SnapError::ZeroTick ) )?;
  let snapped = q.checked_mul( spacing ).ok_or( SnapError::Overflow )?;
  Price::from_minor( snapped ).map_err( | _ | SnapError::Overflow )
}

/// Snap a quantity to the nearest multiple of `lot`, per `rounding`.
///
/// # Errors
///
/// [`SnapError::Overflow`] when the snapped result leaves the representable
/// or declared range. [`SnapError::OffGrid`] when `rounding` is
/// [`Rounding::Exact`] and the quantity is not already on the grid.
pub fn qty_snap_lot( qty : Quantity, lot : Lot, rounding : Rounding ) -> Result< Quantity, SnapError >
{
  let q = exact_round::round_div( qty.minor(), lot.0.minor(), rounding )
  .map_err( | e | round_error_to_snap_error( e, SnapError::ZeroLot ) )?;
  let snapped = q.checked_mul( lot.0.minor() ).ok_or( SnapError::Overflow )?;
  Quantity::from_minor( snapped ).map_err( | _ | SnapError::Overflow )
}
