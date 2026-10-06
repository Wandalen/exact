# 001: SnapError

## Representation

Why a tick/lot could not be constructed, or a snap could not complete. Three
variants: a zero-sized grid for either kind, refused at construction, and a
post-snap range breach.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_snap/src/lib.rs:28-37`

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
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 29-37 | Declaration |
| `src/lib.rs` | 45-47 | Matched in `Display for SnapError` |
| `src/lib.rs` | 84,117 | Constructed in `Tick::new`/`Lot::new` on a zero-sized grid |
| `src/lib.rs` | 54-65 | `round_error_to_snap_error`'s return type and both constructed arms |
| `src/lib.rs` | 149,150,163,164 | Threaded through `price_snap_tick`/`qty_snap_lot`'s error paths |
| `tests/snap_test.rs:12-13` | — | Asserts `Tick::new`/`Lot::new` reject a zero grid with the matching variant |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | The sole error type for the crate's constructors and snap functions; exercised by its own zero-grid test |
| `exact_arith` | `src/lib.rs` | Re-export only — not constructed or matched by the facade's own test suite |
