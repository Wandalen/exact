# 001: TickAt

## Representation

A price grid's spacing at scale `S` — the smallest meaningful price increment a
snapped value may land on. A single-field tuple struct wrapping
`PriceAt< S >`, kept distinct from a bare price so a caller cannot pass an
arbitrary price where a validated, non-zero grid spacing is required. A tick
and the price it snaps share one `S`. [`Tick`](../type_alias/001_tick.md) is
the one at the standard money scale (→ [ADR-001](../../decisions/001_tick_and_lot_generic_over_scale.md)).

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_snap/src/lib.rs:72-76`

```rust
/// A price grid's spacing — the smallest meaningful price increment — at
/// scale `S`. [`Tick`] is the one at the standard money scale.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct TickAt< const S : u32 >( PriceAt< S > );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 76 | Declaration |
| `src/lib.rs` | 79 | The `Tick` alias, `TickAt< MONEY_SCALE >` |
| `src/lib.rs` | 81-109 | `impl TickAt` (constructor and accessor) |
| `src/lib.rs` | 150,161 | `tick` parameter type and field access in `price_snap_tick` |
| `tests/snap_test.rs` | 7,13,22,34,43,52 | Constructed via `Tick::new` and passed into `price_snap_tick` |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs` | 6,40 | A tick in cents, `TickAt::new( cents( "0.05" ) )` |

The derived traits (`Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
Hash`) are not hand-written `impl` blocks and are not cataloged as separate
Implementation instances, per established precedent (`exact_kind`/`exact_ratio`
only catalog their types' hand-written impls).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `src/lib.rs` | The grid-spacing type `price_snap_tick` requires; exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export, and a tick at scale 2 in the any-scale facade test |
