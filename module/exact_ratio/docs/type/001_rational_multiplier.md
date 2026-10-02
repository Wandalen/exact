# Type: Rational Multiplier

### Scope

- **Purpose**: Define what a `Ratio` denotes and what its normalized form guarantees, so a caller can reason about `n / d` as a single always-valid multiplier rather than two independent integers.
- **Responsibility**: The `Ratio` struct, its normalization at construction, and its two accessors.
- **In Scope**: Representation and construction of `Ratio` itself.
- **Out of Scope**: The widened multiply and the division it drives (→ [Widened Multiply Before Narrow](../algorithm/001_widened_multiply_before_narrow.md)); why `RatioError` carries no `ScaleMismatch`/`BadRounding` (→ [Ratio Error Without Scale Mismatch Or Bad Rounding](../decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md)); the conserved value types a `Ratio` multiplies and divides (→ [`exact_kind`](../../../exact_kind/readme.md)).

### Definition

A `Ratio` is a rational multiplier `n / d`, held as two `i64` fields. The
denominator is always stored strictly positive: `ratio_new` accepts a
negative `d` and normalizes it by negating both fields, so `3 / -4` is
constructed as `n = -3, d = 4` rather than stored as supplied. A negative
ratio is therefore always a negative numerator over a positive denominator —
one sign case for every later rounding calculation to handle instead of two.

There is no unchecked constructor. `ratio_new` is the only way to build a
`Ratio`, and it refuses a zero denominator outright rather than deferring the
failure to the first multiply or divide that uses it.

### Construction

| Call | Result |
|------|--------|
| `ratio_new(n, 0)` | `Err(RatioError::DivZero)` |
| `ratio_new(n, d)`, `d > 0` | `Ok(Ratio { n, d })` — unchanged |
| `ratio_new(n, d)`, `d < 0` | `Ok(Ratio { n: -n, d: -d })` — normalized |
| `ratio_new(n, i64::MIN)` | `Err(RatioError::Overflow)` — negating `i64::MIN` has no representable value |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:76-99` | The `Ratio` struct and its `n()`/`d()` accessors |
| `src/lib.rs:101-125` | `ratio_new` — refusal and normalization |

### Tests

| File | Relationship |
|------|--------------|
| `tests/ratio_and_div_round_test.rs` | `ratio_new_refuses_a_zero_denominator`, `ratio_new_normalizes_a_negative_denominator` |
