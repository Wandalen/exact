# 008: Qty::MAX

## Representation

The largest quantity this type can hold, wrapping `Decimal::MAX` — exactly
the declared ceiling. `Qty` has no `MIN`: its floor is always `ZERO`.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:426`

```rust
pub const MAX : Self = Self { value : Decimal::MAX };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 426 | Declaration |
| `tests/non_negative_test.rs` | throughout | Ceiling-boundary checks |
| `exact_add/src/lib.rs:129` | — | **Production** — `qty_saturating_add`'s clamp target |
| `exact_add/tests/checked_and_saturating_add_test.rs:77` | — | Saturation test |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own ceiling-boundary tests |
| `exact_add` | `src/lib.rs` | **Production** — `qty_saturating_add`'s sole clamp target (quantities never overflow negatively, so there is no `Qty::MIN` to pair with it) |
