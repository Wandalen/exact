# 001: RatioError

## Representation

Why a `Ratio` could not be constructed, or a multiply/divide could not be
completed. Four variants: `DivZero` (a zero denominator or divisor),
`Overflow` (the widened product, or the result, left the representable or
declared range), `Negative { minor }` (the result would be below zero, for a
kind that refuses it — carries the minor-unit count the operation would have
produced), `Inexact` (`Rounding::Exact` was asked and the result needed
rounding — mapped from `exact_round::RoundError::Inexact`). Deliberately narrower than the preferred design's own listing,
which also names `ScaleMismatch` and `BadRounding`: `ScaleMismatch` is
unreachable because two different `SCALE` values are two different Rust
types, caught at compile time; `BadRounding` is unreachable because
`exact_round::Rounding` is a closed eight-variant enum with no invalid value
constructible through the public API. `Negative` is this crate's own addition
in their place, for the one real failure the doc's listing missed, and
`Inexact` a second, for `Rounding::Exact`'s refusal (module doc comment,
`src/lib.rs:14-27`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_ratio/src/lib.rs:52`

```rust
pub enum RatioError
{
  DivZero,
  Overflow,
  Negative
  {
    minor : i64,
  },
  Inexact,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 52,68,82,84,88-90,119,123-125,140,144,148-149,155,167,177,192,203,209,221,235,253 | Declaration; `Display`/`Error` impls; `kind_error_to_ratio_error`'s parameter, match arms and return type; `round_error_to_ratio_error`'s match arms and return type; every fallible function's `Result` error type |
| `tests/ratio_and_div_round_test.rs` | 16,58,64-67,131,305,315,317,336-339 | `DivZero` and `Negative` asserted directly, and every variant's message |
| `exact_arith/src/lib.rs:106` | — | Facade re-export |

Doc-comment mentions (lines 16,23,25,137,174-175,187,189-190,218-219,231-233,249-250, all `///`/`//!`
prose) are excluded above — they do not resolve to the declaration.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | The error type for every fallible operation this crate exposes |
| `exact_arith` | `src/lib.rs` | Re-export only |
