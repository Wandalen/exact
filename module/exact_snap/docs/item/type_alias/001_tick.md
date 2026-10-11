# 001: Tick

## Representation

A tick at the standard money scale — `TickAt< MONEY_SCALE >`. The name the
exchange stores (`exchange_spec`'s `pub tick : Tick`), kept working when the
struct itself became generic over the scale
(→ [ADR-001](../../decisions/001_tick_and_lot_generic_over_scale.md)).

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_snap/src/lib.rs:79`

```rust
pub type Tick = TickAt< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 79 | Declaration |
| `src/lib.rs` | 17-21 | Module doc example: a tick of 5 minor units |
| `tests/snap_test.rs` | 7,13,22,34,43,52,61,73,100,101,146,158,159,160,193 | Constructed via `Tick::new` at the standard scale |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:49,195` | — | The demo lane's tick size, through `exact_arith` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | Declared here; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export |
| `smoke_exact_market_split` | `src/lib.rs` | A tick at the standard scale |
