# 001: use exact_kind::{ Decimal, KindError, Money, PriceAt, Qty }

## Representation

Imports the three conserved-value types, in their generic form, their shared
error enum, and `Money` from `exact_kind` — every function in this crate
parses text into exactly one of `Decimal< S >`/`Qty< S >`/`PriceAt< S >` at
any scale (→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)) and returns `KindError` on failure; `Money` is named only
by the compile-time scale-consistency assertion.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_parse/src/lib.rs:39`

```rust
use exact_kind::{ Decimal, KindError, Money, PriceAt, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 39 | Declaration — brings all 5 names into scope for every function signature and the compile-time assertion in the file |
| `tests/from_str_test.rs:5` | — | Re-imports only `KindError` directly from `exact_kind` for the negative/malformed assertions; does not re-import `Money`/`Price`/`Quantity` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Every public function's parameter and return types, and the cross-crate scale assertion's `Money::ONE_MINOR` reference |
