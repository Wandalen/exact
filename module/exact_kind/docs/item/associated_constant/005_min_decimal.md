# 005: Decimal::MIN

## Representation

The smallest (most negative) value this type can hold — the negated declared
ceiling. Exists only on `Decimal`: `Qty` has no `MIN` (its floor is always
`ZERO`, already provided).

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:177`

```rust
pub const MIN : Self = Self { minor : -CEILING_MINOR_UNITS };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 177 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Floor-boundary checks |
| `exact_add/src/lib.rs:115` | — | **Production** — `money_saturating_add`'s negative clamp target |
| `exact_add/tests/checked_and_saturating_add_test.rs:61` | — | Saturation test |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own floor-boundary tests |
| `exact_add` | `src/lib.rs` | **Production** — `money_saturating_add`'s clamp target when the checked add overflows negatively |
