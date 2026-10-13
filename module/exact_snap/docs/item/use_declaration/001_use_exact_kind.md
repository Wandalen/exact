# 001: use exact_kind::{ PriceAt, Qty }

## Representation

Brings the two conserved-value types this crate snaps into scope, in their
generic form — every item here takes its scale `S` as a parameter
(→ [ADR-001](../../decisions/001_tick_and_lot_generic_over_scale.md)).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_snap/src/lib.rs:24`

```rust
use exact_kind::{ PriceAt, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 24 | Declaration |
| `src/lib.rs` | 76,89,96,105,150,165 | `PriceAt` in `TickAt`'s tuple field, `TickAt::new`'s parameter and magnitude, `TickAt::price`'s return type, `price_snap_tick`'s parameter/return types and result |
| `src/lib.rs` | 114,126,137,175,180 | `Qty` in `LotAt`'s tuple field, `LotAt::new`'s parameter, `LotAt::qty`'s return type, `qty_snap_lot`'s parameter/return types and result |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | Underlying types for both grid-spacing wrappers and both snap functions |
