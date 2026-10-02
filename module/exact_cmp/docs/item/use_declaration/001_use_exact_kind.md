# 001: use exact_kind::{ Money, Price, Quantity }

## Representation

Imports the three conserved value types every comparison/equality/min-max
function in this crate operates on.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_cmp/src/lib.rs:20`

```rust
use exact_kind::{ Money, Price, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 20 | Declaration |
| `tests/cmp_test.rs` | 6 | Identical import, for the same three types |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_cmp` | `(defining crate)` | Every function signature in the crate names at least one of these three types |
