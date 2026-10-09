# 002: use exact_round::Rounding

## Representation

Brings in the eight-variant rounding-mode enum that every multiply
(`*_mul_ratio`, `price_mul_qty`, and the private `mul_ratio_minor` they share)
and every `*_div_round` function (and the private `div_round_minor` they
share) takes as a parameter. The multiplies thread it through to
`exact_round::round_div_wide` (`src/lib.rs:165-166`), the divisions to
`exact_round::round_div` (`src/lib.rs:211`).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_ratio/src/lib.rs:48`

```rust
use exact_round::Rounding;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 48,155,177,192,203,209,221,235,253 | Parameter type of `mul_ratio_minor` and `div_round_minor` (both private), the three `*_mul_ratio` functions, both `*_div_round` functions and `price_mul_qty` |

Doc-comment mentions at lines 20 (`` [`exact_round::Rounding`] ``) and 26 are
prose, not usages, and are excluded above. The intra-doc links
`` [`Rounding::Exact`] `` at lines 64,176,190,220,234,251 resolve through this
declaration.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Mode parameter for every multiply and rounding-division function |
