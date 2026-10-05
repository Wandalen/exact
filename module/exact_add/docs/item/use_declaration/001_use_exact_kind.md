# 001: use exact_kind::{ KindError, Money, Price, Quantity }

## Representation

Imports the four conserved-value types and their shared error enum from
`exact_kind` — every function in this crate operates on `Money`/`Quantity`/
`Price` and returns `KindError` on failure, so all four names are needed at
the crate's single `use` site.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_add/src/lib.rs:46`

```rust
use exact_kind::{ KindError, Money, Price, Quantity };
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
