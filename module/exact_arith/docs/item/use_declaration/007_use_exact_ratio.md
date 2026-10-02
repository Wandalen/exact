# 007: pub use exact_ratio::{ ... }

## Representation

Re-exports `exact_ratio`'s `Ratio` type, its error enum, and all 6 of its
multiply/divide functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:96`

```rust
pub use exact_ratio::{ Ratio, RatioError, money_div_round, money_mul_ratio, price_mul_ratio, qty_div_round, qty_mul_ratio, ratio_new };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 96 | Declaration |

Confirmed via a full-workspace grep: not one of these 7 names is imported
or called through `exact_arith` anywhere, including this crate's own test
suite — matching `exact_ratio`'s own catalog finding that `price_mul_ratio`/
`qty_div_round` have zero callers even within `module/` itself; at
the facade layer, the same now holds for all 7 names in this block.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
