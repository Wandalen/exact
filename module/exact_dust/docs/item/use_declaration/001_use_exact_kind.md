# 001: use exact_kind::{ KindError, Money, Quantity }

## Representation

Brings the two conserved-value types this crate splits into scope, and
`KindError`, the error type of the `from_minor` constructor the generic
helpers take as `make`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_dust/src/lib.rs:60`

```rust
use exact_kind::{ KindError, Money, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 60 | Declaration |
| `src/lib.rs` | 161,179 | `KindError` in the `make : fn( i64 ) -> Result< T, KindError >` parameter of `split_with` and `split_into_with` (private) |
| `src/lib.rs` | 207,209,218,220,231,243,245,253,255,263 | `Money`/`Quantity` parameter and return types on the 6 public split/remainder functions, and `Money::from_minor`/`Quantity::from_minor` passed as `make` |

`KindError` is named only in the type of `make`; no `KindError` value
leaves this crate. Every one is folded into `DustError::Overflow` at the
`.map_err(|_| DustError::Overflow)` boundary (`src/lib.rs:166,194`).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Parameter/return types for all 6 public functions; `KindError` in the generic helpers' `make` type |
