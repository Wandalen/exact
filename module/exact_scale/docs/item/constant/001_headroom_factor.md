# 001: HEADROOM_FACTOR

## Representation

The headroom the range budget requires between the ceiling and the width —
`1000`, about ten bits. A magnitude allowance for intermediates: it makes
accumulating a thousand ceiling-sized amounts safe (the shape of an audit's
inner fold), but does not make multiply-before-divide safe.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_scale/src/lib.rs:22`

```rust
pub const HEADROOM_FACTOR : i64 = 1000;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 22,49 | Declaration; the crate's own compile-time range-budget assert |
| `tests/scale_factor_test.rs` | 3,25,27 | Headroom-relation test |
| `exact_arith/src/lib.rs:82` | — | Facade re-export only |
| `exact_kind/tests/checked_arithmetic_test.rs:16,180,183` | — | Re-derives the same headroom check inside `exact_kind`'s own test suite |

No production (non-test, non-facade) file outside `exact_scale` reads
`HEADROOM_FACTOR` — an honest gap: the headroom relation is checked once,
at declaration, and the only other place it is re-checked is a test.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_scale` | `(defining crate)` | Bounds `CEILING_MINOR_UNITS` via the compile-time assert |
| `exact_arith` | `src/lib.rs` | Re-export only |
| `exact_kind` | `tests/checked_arithmetic_test.rs` | Test-only re-verification of the same headroom relation |
