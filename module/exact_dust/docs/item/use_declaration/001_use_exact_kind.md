# 001: use exact_kind::{ Decimal, KindError, Qty }

## Representation

Brings the two conserved-value types this crate splits into scope, in their
generic form (→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)), and
`KindError`, the error type of the `from_minor` constructor the generic
helpers take as `make`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_dust/src/lib.rs:62`

```rust
use exact_kind::{ Decimal, KindError, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 62 | Declaration |
| `src/lib.rs` | 164,182 | `KindError` in the `make : fn( i64 ) -> Result< T, KindError >` parameter of `split_with` and `split_into_with` (private) |
| `src/lib.rs` | 210,212,221,223,234,246,248,256,258,266,320,335,340 | `Decimal< S >`/`Qty< S >` parameter and return types on the 6 public split/remainder functions, and `Decimal::< S >::from_minor`/`Qty::< S >::from_minor` passed as `make`; `Decimal< S >` in and out of `money_dust_split_weighted` |

`KindError` is named only in the type of `make`; no `KindError` value
leaves this crate. Every one is folded into `DustError::Overflow` at the
`.map_err(|_| DustError::Overflow)` boundary (`src/lib.rs:169,197`).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Parameter/return types for all 7 public functions; `KindError` in the generic helpers' `make` type |
