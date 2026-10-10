# 001: use exact_kind::{ KindError, Money, Price, Quantity }

## Representation

Brings in the three conserved value types this crate's multiply/divide
functions operate over, plus the shared error type every `exact_kind`
constructor returns, which this crate's own `kind_error_to_ratio_error`
remaps into a `RatioError`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_ratio/src/lib.rs:47`

```rust
use exact_kind::{ KindError, Money, Price, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 47,84,88-90,177,180,192,195,203,206,221,224,235,238 | `KindError` is `kind_error_to_ratio_error`'s parameter type and match subject, each of its five variants matched by name (80,84-86); `Money`/`Price`/`Quantity` are the operand and return types of the 5 `*_mul_ratio`/`*_div_round` functions |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Operand/return types for every multiply and divide function, and the error-mapping target |
