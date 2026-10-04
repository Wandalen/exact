# 004: CEILING_MINOR_UNITS

## Representation

The declared ceiling on the stored integer, at every scale — derived from
`CEILING_WHOLE_UNITS` at `MONEY_SCALE`, then applied to the stored count of
minor units whatever the scale is, because that is the quantity that
overflows, not the value it denotes.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_scale/src/lib.rs:45`

```rust
pub const CEILING_MINOR_UNITS : i64 = CEILING_WHOLE_UNITS * pow10( MONEY_SCALE );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 45,49 | Declaration; the crate's own range-budget assert |
| `tests/scale_factor_test.rs` | throughout | Headroom-relation and cross-check tests |
| `exact_arith/src/lib.rs:81` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:54` | — | Cross-check via the re-exported name |
| `exact_kind/src/lib.rs:125,189,192,201` | — | **Production** — rendered in `KindError::Display`; defines `Decimal::MAX`/`MIN`; the range gate in `from_minor` |
| `exact_kind/tests/checked_arithmetic_test.rs` | throughout | Ceiling-boundary checks |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_scale` | `(defining crate)` | Bounded by `HEADROOM_FACTOR`'s compile-time assert; exercised by its own test |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; its own test cross-checks it via the re-exported name |
| `exact_kind` | `src/lib.rs` | **Production** — the actual runtime range gate behind every `Decimal`/`Qty` constructor, via `from_minor` |
