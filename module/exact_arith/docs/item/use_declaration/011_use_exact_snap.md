# 011: pub use exact_snap::{ ... }

## Representation

Re-exports `exact_snap`'s `Tick`/`Lot` grid types, its error enum, and both
snap functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:117`

```rust
pub use exact_snap::{ Lot, SnapError, Tick, price_snap_tick, qty_snap_lot };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 117 | Declaration |

Confirmed via a full-workspace grep: not one of these 5 names is imported
or called through `exact_arith` anywhere, including this crate's own test
suite — matching `exact_snap`'s own catalog finding that both its functions
have zero callers even within `module/` itself.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
