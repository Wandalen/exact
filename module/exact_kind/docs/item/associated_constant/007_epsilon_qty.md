# 007: Qty::EPSILON

## Representation

The smallest non-zero quantity this type can express, wrapping
`Decimal::EPSILON`.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:408`

```rust
pub const EPSILON : Self = Self { value : Decimal::EPSILON };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 408 | Declaration |
| `tests/non_negative_test.rs` | throughout | Boundary-adjacent values (`ZERO.checked_sub(EPSILON)`, `MAX.checked_add(EPSILON)`) |
| `exact_add/tests/checked_and_saturating_add_test.rs:77` | — | Saturation-boundary test input |

No production (non-test) file outside `exact_kind` uses `Quantity::EPSILON`
directly — an honest gap, matching [Decimal::EPSILON](003_epsilon_decimal.md).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own boundary tests |
| `exact_add` | `tests/*.rs` | Test-only boundary input |
