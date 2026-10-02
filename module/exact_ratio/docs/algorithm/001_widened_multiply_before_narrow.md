# Algorithm: Widened Multiply Before Narrow

### Scope

- **Purpose**: State exactly how a ratio multiply avoids both double-rounding and intermediate overflow, so a caller can predict that an operand and result that each fit `i64` never fails on the product alone.
- **Responsibility**: `mul_ratio_minor`, the shared core of `money_mul_ratio`, `qty_mul_ratio`, and `price_mul_ratio`.
- **In Scope**: The order of multiply-then-divide, and the intermediate width that order needs.
- **Out of Scope**: The division `div_round` drives, which performs no multiply and needs no widening (→ [`exact_round`](../../../exact_round/readme.md)); the shape of `Ratio` itself (→ [Rational Multiplier](../type/001_rational_multiplier.md)).

### Procedure

1. Widen `minor` (the value's own minor-unit count) and `r.n` (the ratio's
   numerator) to `i128`.
2. Multiply the two widened values — this product is what must not overflow
   the intermediate.
3. Divide the widened product by `i128::from(r.d)`.
4. Narrow the quotient back to `i64` via `try_from`, returning
   `RatioError::Overflow` when it does not fit.
5. Hand the narrowed minor-unit count to the kind's own `from_minor`, which
   applies the declared-ceiling check on top.

### Why Multiply Before Divide

A rate stated as `n / d` must multiply by `n` before dividing by `d` to stay
exact. Dividing first (`minor / d * n`) rounds the intermediate quotient
before the multiply ever runs, discarding a remainder the correct order
would have kept. Multiplying first and dividing once is the only order that
rounds at most one time.

### Why The Product Needs `i128`, Not `i64`

Multiplying first means computing `minor * r.n` before any division narrows
it back down. A value already near the declared ceiling, times a
free-standing numerator with no relationship to the ceiling, can overflow
`i64` long before the final quotient — or either operand alone — would. The
test below exercises exactly this: an identity ratio (`1_000_000 /
1_000_000`) applied to `Money::MAX` succeeds, even though
`Money::MAX.minor() * 1_000_000` does not fit in `i64` (`Money::MAX.minor()`
is `9 × 10^15`; the product is `9 × 10^21`, far past `i64::MAX`), because the
multiply itself runs in `i128`.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:127-132` | `mul_ratio_minor` — the widen, multiply, divide, narrow sequence |
| `src/lib.rs:134-169` | `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio` — the three callers, one per kind |

### Tests

| File | Relationship |
|------|--------------|
| `tests/ratio_and_div_round_test.rs` | `mul_ratio_survives_an_intermediate_that_would_overflow_i64` — the motivating case; `mul_ratio_by_one_half_halves_the_value` — the ordinary case |
