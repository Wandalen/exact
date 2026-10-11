# 001: use exact_kind::{ Decimal, KindError, PriceAt, Qty }

## Representation

Brings in the three conserved value types this crate's multiply/divide
functions operate over, in their generic form (→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)), plus the shared error type every `exact_kind`
constructor returns, which this crate's own `kind_error_to_ratio_error`
remaps into a `RatioError`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_ratio/src/lib.rs:47`

```rust
use exact_kind::{ Decimal, KindError, PriceAt, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 47,84,88-90,177,180,192,195,203,206,221,224,235,238,256,259,261 | `KindError` is `kind_error_to_ratio_error`'s parameter type and match subject, each of its five variants matched by name (80,84-86); `Decimal`/`PriceAt`/`Qty` are the operand and return types of the 5 `*_mul_ratio`/`*_div_round` functions, and of `price_mul_qty` at its two scales |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Operand/return types for every multiply and divide function, and the error-mapping target |
