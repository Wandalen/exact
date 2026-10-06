# 001: use exact_kind::{ Price, Quantity }

## Representation

Brings the two conserved-value types this crate snaps into scope.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_snap/src/lib.rs:24`

```rust
use exact_kind::{ Price, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 24 | Declaration |
| `src/lib.rs` | 70,96,136 | `Price` in `Tick`'s tuple field, `Tick::price`'s return type, `price_snap_tick`'s parameter/return types |
| `src/lib.rs` | 104,124,160 | `Quantity` in `Lot`'s tuple field, `Lot::qty`'s return type, `qty_snap_lot`'s parameter/return types |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | Underlying types for both grid-spacing wrappers and both snap functions |
