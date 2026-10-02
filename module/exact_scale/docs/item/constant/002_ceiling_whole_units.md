# 002: CEILING_WHOLE_UNITS

## Representation

The declared maximum holdings of one asset kind, in whole units —
9,000,000,000. A deployment input this crate cannot derive; what it can do is
make it a declared constant with a checked relationship to the backing
width.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_scale/src/lib.rs:29`

```rust
pub const CEILING_WHOLE_UNITS : i64 = 9_000_000_000;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 29,45 | Declaration; `CEILING_MINOR_UNITS`'s own definition |
| `tests/scale_factor_test.rs:3,21` | — | Cross-check against `CEILING_MINOR_UNITS` |
| `exact_arith/src/lib.rs:75` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:11,54` | — | Cross-check via the re-exported name |
| `exact_kind/tests/checked_arithmetic_test.rs:16,179` | — | Cross-check, same pattern as `exact_scale`'s own test |
| `exact_kind/tests/non_negative_test.rs:9,46,84,87,112,135` | — | Whole-unit ceiling boundary for `Quantity` |

No production (non-test, non-facade) file outside `exact_scale` reads
`CEILING_WHOLE_UNITS` directly — every real consumer reaches the ceiling
through [CEILING_MINOR_UNITS](004_ceiling_minor_units.md) instead, which is
the quantity actually checked against at runtime.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_scale` | `(defining crate)` | Defines `CEILING_MINOR_UNITS`; exercised by its own cross-check test |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; its own test cross-checks it via the re-exported name |
| `exact_kind` | `tests/*.rs` | Test-only — whole-unit boundary input for `Quantity` |
