# 007: pub use exact_ratio::{ ... }

## Representation

Re-exports `exact_ratio`'s `Ratio` type, its error enum, and all 7 of its
multiply/divide functions, `price_mul_qty` included.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:102`

```rust
pub use exact_ratio::{ Ratio, RatioError, money_div_round, money_mul_ratio, price_mul_qty, price_mul_ratio, qty_div_round, qty_mul_ratio, ratio_new };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 102 | Declaration |
| `src/lib.rs` | 23,28 | Module doc example — `price_mul_qty` |
| `tests/facade_test.rs` | 9,25 | `price_mul_qty`, through the facade |

Through `exact_arith`, only `price_mul_qty` is used — by this crate's own
doc example and facade test. The other 8 names are not imported or called
through the facade anywhere, matching `exact_ratio`'s own catalog finding
that `price_mul_ratio`/`qty_div_round` have zero callers even within
`module/` itself.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | `price_mul_qty` exercised by its doc example and facade test; the rest declared only |
