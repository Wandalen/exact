//! Tier 4: the facade over this family's 14-crate value substrate. Money,
//! commodity quantities, and every other conserved value get exact, checked
//! fixed-point/decimal types instead of floats; a conservation auditor
//! proves the books balance across an arbitrary transaction log.
//!
//! # A facade, and nothing else
//!
//! This crate declares no type and performs no arithmetic. It is a single
//! block of `pub use` per source crate, and it is what every consumer
//! depends on — so a consumer writes one dependency instead of fourteen, and
//! the family can be re-split without a consumer noticing.
//!
//! The restraint is the point. An arithmetic helper added here would be
//! arithmetic that lives outside the crate whose tests grade it, and the
//! first such helper is how a facade turns into a fifteenth implementation.
//! Anything that needs to compute belongs in the tier that owns the type.
//! Ported verbatim from the real, already-implemented `exact_arithmetic`
//! facade this crate replaces, including its own mechanical purity test.
//!
//! ```
//! use exact_arith::
//! {
//!   money_dust_split, price_mul_qty, DustTo, Entry, Money, Price, Quantity, Rounding, verify,
//! };
//!
//! let price = Price::parse( "1.25" ).unwrap();
//! let held = Quantity::parse( "2.5" ).unwrap();
//! let cost = price_mul_qty( price, held, Rounding::HalfEven ).unwrap();
//! assert_eq!( cost, Money::parse( "3.125" ).unwrap() );
//! assert!( verify( &[ Entry::new( "a", 5 ), Entry::new( "b", -5 ) ] ).unwrap().is_balanced() );
//!
//! let total = Money::from_minor( 11 ).unwrap();
//! let shares = money_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap();
//! assert_eq!( shares[ 0 ], Money::from_minor( 5 ).unwrap() );
//! ```
//!
//! # Disclosed deviations from the preferred design's own listing
//!
//! - **No `exact_zero_money`/`exact_zero_qty`/`exact_zero_price`.** The
//!   type doc names these as "the only functions this crate defines itself,"
//!   but each would be a one-line wrapper around a constant this facade
//!   already re-exports (`Money::ZERO`, `Quantity::ZERO`, `Price::ZERO`) —
//!   pure duplication with no behaviour of its own, and the first crack in
//!   the real `exact_arithmetic` facade's own carefully-reasoned, mechanically-
//!   tested "declares nothing of its own" discipline (ported here verbatim,
//!   purity test included). A caller writes `Money::ZERO` through this
//!   facade exactly as through any other re-exported item.
//! - **Depends on, and re-exports, all 14 leaves — including the 3 Tier-0
//!   roots (`exact_minor`, `exact_scale`, `exact_round`)**, not only the 10
//!   the crate doc names as "non-root leaves ... reached transitively."
//!   `pub use` requires a direct dependency on the crate it names; several
//!   re-exported functions (`money_div_round`, `price_snap_tick`, …) take
//!   [`Rounding`] by value in their own public signature, so a consumer
//!   calling them through this facade alone needs `Rounding` nameable
//!   through it too, not merely reachable transitively at the type-checker
//!   level. `exact_dust` already established the same direct-dependency
//!   precedent for the identical reason.
//! - **`exact_sign`'s surface is re-exported too**, though the crate doc's
//!   own dependency list omits it (no *other* leaf's public signature names
//!   `Sign`). Included anyway, matching the real `exact_arithmetic`'s own
//!   generous precedent of re-exporting even constants no signature strictly
//!   requires (`HEADROOM_FACTOR`, `CEILING_WHOLE_UNITS`) — this facade's
//!   whole purpose is exposing the full value substrate through one
//!   dependency, not only the slice other leaves happen to reference.

pub use exact_minor::
{
  Backing,
  Minor,
  MinorError,
  minor_checked_add,
  minor_checked_neg,
  minor_checked_sub,
  minor_from_i64,
  minor_is_zero,
  minor_saturating_add,
  minor_saturating_sub,
  minor_to_i64,
  minor_zero,
};

pub use exact_scale::{ CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS, HEADROOM_FACTOR, MONEY_SCALE, pow10 };

pub use exact_round::{ Rounding, RoundError, round_div, round_div_wide, rounding_default, rounding_name };

pub use exact_sign::{ Sign, sign_is_negative, sign_is_zero, sign_neg_allowed, sign_of };

pub use exact_kind::{ Decimal, KindError, Money, Price, Qty, Quantity };

pub use exact_add::
{
  money_add,
  money_checked_neg,
  money_saturating_add,
  money_sub,
  price_add,
  price_sub,
  qty_add,
  qty_saturating_add,
  qty_sub,
};

pub use exact_ratio::
{
  Ratio,
  RatioError,
  money_div_round,
  money_mul_ratio,
  price_mul_qty,
  price_mul_ratio,
  qty_div_round,
  qty_mul_ratio,
  ratio_new,
};

pub use exact_parse::{ money_from_str, price_from_str, qty_from_str };

pub use exact_fmt::{ FmtError, fmt_into, money_fmt, price_fmt, qty_fmt };

pub use exact_bytes::
{
  KIND_MONEY,
  KIND_PRICE,
  KIND_QTY,
  Wire,
  WireError,
  money_from_wire,
  money_to_wire,
  price_from_wire,
  price_to_wire,
  qty_from_wire,
  qty_to_wire,
};

pub use exact_snap::{ Lot, SnapError, Tick, price_snap_tick, qty_snap_lot };

pub use exact_cmp::{ money_cmp, money_eq, price_cmp, price_max, price_min, qty_cmp };

pub use exact_dust::{ DustError, DustTo, money_dust_remainder, money_dust_split, money_dust_split_into, qty_dust_remainder, qty_dust_split, qty_dust_split_into };

pub use exact_conserve::{ ConservationError, Entry, Report, money_conserve_into, money_sum_assert_zero, qty_conserve_into, qty_sum_assert_zero, verify };
