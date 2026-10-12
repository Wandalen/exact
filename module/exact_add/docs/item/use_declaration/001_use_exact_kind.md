# 001: use exact_kind::{ Decimal, KindError, PriceAt, Qty }

## Representation

Imports the three conserved-value types, in their generic form, and their
shared error enum from `exact_kind` — every function in this crate operates
on `Decimal< S >`/`Qty< S >`/`PriceAt< S >` at any scale (→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)) and
returns `KindError` on failure, so all four names are needed at the crate's
single `use` site.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_add/src/lib.rs:46`

```rust
use exact_kind::{ Decimal, KindError, PriceAt, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 46 | Declaration — brings all 4 names into scope for every function signature in the file |
| `tests/checked_and_saturating_add_test.rs:9` | — | Re-imports the same 4 names directly from `exact_kind` for test assertions |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Every public function's parameter and return types |
