# 001: use exact_kind::{ KindError, Money, Quantity }

## Representation

Brings in the conserved value types and their error type for the typed
per-kind convenience layer (`money_conserve_into`, `qty_conserve_into`,
`money_sum_assert_zero`, `qty_sum_assert_zero`). `Entry`, `Report`, and
`verify` — the carried-forward `exact_audit` surface — touch none of these;
they stay dependency-free in their own logic exactly as before, per the
module doc comment's disclosed deviation (`src/lib.rs:40-49`).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_conserve/src/lib.rs:77`

```rust
use exact_kind::{ KindError, Money, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 77 | Declaration |
| `src/lib.rs` | 134 | `KindError` — private error-mapping function's parameter |
| `src/lib.rs` | 223, 234 | `Money`/`Quantity` — `money_conserve_into`/`qty_conserve_into` parameter types |
| `src/lib.rs` | 245, 273 | `Money`/`Quantity` — `money_sum_assert_zero`/`qty_sum_assert_zero` slice element types |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Parameter/element types for the 4 typed convenience functions |
