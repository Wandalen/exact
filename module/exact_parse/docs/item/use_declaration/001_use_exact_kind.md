# 001: use exact_kind::{ KindError, Money, Price, Quantity }

## Representation

Imports the three conserved-value types and their shared error enum from
`exact_kind` — every function in this crate parses text into exactly one of
`Money`/`Quantity`/`Price` and returns `KindError` on failure, so all four
names are needed at the crate's single `use` site.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_parse/src/lib.rs:29`

```rust
use exact_kind::{ KindError, Money, Price, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 29 | Declaration — brings all 4 names into scope for every function signature and the compile-time assertion in the file |
| `tests/from_str_test.rs:5` | — | Re-imports only `KindError` directly from `exact_kind` for the negative/malformed assertions; does not re-import `Money`/`Price`/`Quantity` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Every public function's parameter and return types, and the cross-crate scale assertion's `Money::ONE_MINOR` reference |
