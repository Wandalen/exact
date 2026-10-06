# 002: use exact_round::Rounding

## Representation

Brings in the three-variant rounding-mode enum that every multiply
(`*_mul_ratio`, `price_mul_qty`, and the private `mul_ratio_minor` they share)
and every `*_div_round` function (and the private `div_round_minor` they
share) takes as a parameter. The multiplies thread it through to
`exact_round::round_div_wide` (`src/lib.rs:150-151`), the divisions to
`exact_round::round_div` (`src/lib.rs:193`).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_ratio/src/lib.rs:47`

```rust
use exact_round::Rounding;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 47,140,161,174,185,191,206,219,235 | Parameter type of `mul_ratio_minor` and `div_round_minor` (both private), the three `*_mul_ratio` functions, both `*_div_round` functions and `price_mul_qty` |

A doc-comment mention at line 20 (`` [`exact_round::Rounding`] ``) is prose,
not a usage, and is excluded above.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Mode parameter for every multiply and rounding-division function |
