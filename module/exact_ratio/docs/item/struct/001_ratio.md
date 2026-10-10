# 001: Ratio

## Representation

A rational multiplier `n / d`, with `d` always stored positive. A negative
ratio is conventionally represented as a negative numerator over a positive
denominator — `ratio_new` normalizes any negative `d` on construction so
every later rounding calculation needs only one sign case instead of two
(module doc comment, `src/lib.rs:27-32`). Both fields are private; `n()` and
`d()` are the only read access.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_ratio/src/lib.rs:96`

```rust
pub struct Ratio
{
  n : i64,
  d : i64,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 96,102,140,150,152,155,177,192,203 | Declaration; inherent impl; `ratio_new`'s return type and construction; parameter type of `mul_ratio_minor` and all 3 `*_mul_ratio` functions |
| `exact_arith/src/lib.rs:105` | — | Facade re-export |

No test file names `Ratio` directly — every test constructs one through
`ratio_new` and holds it via type inference. Doc-comment mentions (lines
1,26) are prose and excluded above.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | The multiplier operand type for every multiply function |
| `exact_arith` | `src/lib.rs` | Re-export only |
