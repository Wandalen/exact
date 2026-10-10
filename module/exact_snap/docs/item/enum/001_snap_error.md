# 001: SnapError

## Representation

Why a tick/lot could not be constructed, or a snap could not complete. Four
variants: a zero-sized grid for either kind, refused at construction, a
post-snap range breach, and `OffGrid` — under `Rounding::Exact`, a value not
already on the grid, refused rather than snapped (mapped from
`exact_round::RoundError::Inexact`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_snap/src/lib.rs:28-39`

```rust
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
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 29-39 | Declaration |
| `src/lib.rs` | 47-50 | Matched in `Display for SnapError` |
| `src/lib.rs` | 88,121 | Constructed in `Tick::new`/`Lot::new` on a zero-sized grid |
| `src/lib.rs` | 57-69 | `round_error_to_snap_error`'s return type and its constructed arms (`Overflow`, `OffGrid`) |
| `src/lib.rs` | 154,155,169,170 | Threaded through `price_snap_tick`/`qty_snap_lot`'s error paths |
| `tests/snap_test.rs:13-14` | — | Asserts `Tick::new`/`Lot::new` reject a zero grid with the matching variant |
| `tests/snap_test.rs:199,203,210` | — | `OffGrid` under `Exact`, for a price and a quantity, and its message |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | The sole error type for the crate's constructors and snap functions; exercised by its own zero-grid test |
| `exact_arith` | `src/lib.rs` | Re-export only — not constructed or matched by the facade's own test suite |
