# 002: LotAt

## Representation

A quantity grid's spacing at scale `S` — the smallest meaningful quantity
increment a snapped value may land on. The quantity counterpart to
[TickAt](001_tick_at.md), same rationale: a single-field tuple struct so a
validated, non-zero grid spacing can't be confused with an arbitrary
`Qty< S >`. [`Lot`](../type_alias/002_lot.md) is the one at the standard money
scale (→ [ADR-001](../../decisions/001_tick_and_lot_generic_over_scale.md)).

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_snap/src/lib.rs:111-114`

```rust
/// A quantity grid's spacing — the smallest meaningful quantity increment —
/// at scale `S`. [`Lot`] is the one at the standard money scale.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct LotAt< const S : u32 >( Qty< S > );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 114 | Declaration |
| `src/lib.rs` | 117 | The `Lot` alias, `LotAt< MONEY_SCALE >` |
| `src/lib.rs` | 119-141 | `impl LotAt` (constructor and accessor) |
| `src/lib.rs` | 175,177,179 | `lot` parameter type and field access in `qty_snap_lot` |
| `tests/snap_test.rs` | 7,14,118,129 | Constructed via `Lot::new` and passed into `qty_snap_lot` |
| `exact_arith/src/lib.rs:136` | — | Facade re-export |

The derived traits are not hand-written `impl` blocks and are not cataloged
separately, per the same precedent noted on [TickAt](001_tick_at.md).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `src/lib.rs` | The grid-spacing type `qty_snap_lot` requires; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |
