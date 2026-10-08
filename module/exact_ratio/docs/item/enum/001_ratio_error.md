# 001: RatioError

## Representation

Why a `Ratio` could not be constructed, or a multiply/divide could not be
completed. Three variants: `DivZero` (a zero denominator or divisor),
`Overflow` (the widened product, or the result, left the representable or
declared range), `Negative { minor }` (the result would be below zero, for a
kind that refuses it — carries the minor-unit count the operation would have
produced). Deliberately narrower than the preferred design's own listing,
which also names `ScaleMismatch` and `BadRounding`: `ScaleMismatch` is
unreachable because two different `SCALE` values are two different Rust
types, caught at compile time; `BadRounding` is unreachable because
`exact_round::Rounding` is a closed seven-variant enum with no invalid value
constructible through the public API. `Negative` is this crate's own addition
in their place, for the one real failure the doc's listing missed (module doc
comment, `src/lib.rs:14-25`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_ratio/src/lib.rs:51`

```rust
pub enum RatioError
{
  DivZero,
  Overflow,
  Negative
  {
    minor : i64,
  },
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 51,65,78,80,84-86,126,130,134-135,141,151-153,162,175,186,192,196-197,207,220,236 | Declaration; `Display`/`Error` impls; `kind_error_to_ratio_error`'s parameter, match arms and return type; every fallible function's `Result` error type |
| `tests/ratio_and_div_round_test.rs` | 16,58,64-67,131,305,315,317,336-339 | `DivZero` and `Negative` asserted directly, and every variant's message |
| `exact_arith/src/lib.rs:106` | — | Facade re-export |

Doc-comment mentions (lines 16,23,122,151,163-164,196,208-209,225, all `///`/`//!`
prose) are excluded above — they do not resolve to the declaration.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | The error type for every fallible operation this crate exposes |
| `exact_arith` | `src/lib.rs` | Re-export only |
