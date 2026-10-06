# 001: use exact_kind::{ Money, Quantity }

## Representation

Brings the two conserved-value types this crate splits into scope.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_dust/src/lib.rs:60`

```rust
use exact_kind::{ Money, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 60 | Declaration |
| `src/lib.rs` | 162,177,203,215,229,253 | `Money`/`Quantity` parameter and return types on the 6 public split/remainder functions |

Does not import `KindError` — unlike `exact_add`/`exact_parse`/`exact_ratio`,
this crate never receives a `KindError` directly; it maps overflow/negative
outcomes through its own `DustError` instead, discarding the specific
`exact_kind` error at the `.map_err(|_| DustError::Overflow)` boundary
(`src/lib.rs:167,191,221,243`).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Parameter/return types for all 6 public functions |
