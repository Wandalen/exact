# Invariant: Widened Product Never Silently Overflows

### Scope

- **Purpose**: State that a ratio multiply's intermediate product cannot wrap or truncate unnoticed, so a caller can trust that failure is always reported as `RatioError::Overflow` rather than a silently wrong answer.
- **Responsibility**: `mul_ratio_minor`, the shared core of `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio`.
- **In Scope**: The width of the intermediate product and the narrowing check after it.
- **Out of Scope**: Why multiply runs before divide at all (→ `../algorithm/001_widened_multiply_before_narrow.md`'s "Why Multiply Before Divide"); `div_round`'s own rounding behavior, which performs no multiply (→ [`exact_round`](../../../exact_round/readme.md)).

### Statement

`mul_ratio_minor` (`src/lib.rs:140-145`) widens both operands to `i128`
before multiplying, so the product itself — `minor * r.n`, computed before
any division narrows it — can never silently wrap: `i128` holds the full
product of any two `i64` values with room to spare. The result is narrowed
back to `i64` only once, via `i64::try_from`, which fails loudly as
`RatioError::Overflow` rather than truncating when the final quotient does
not fit. An operand and a result that each individually fit `i64` are
therefore never defeated by an intermediate that would not have.

### Rationale

Multiplying before dividing is what keeps a ratio multiply exact (see the
algorithm doc's own "Why Multiply Before Divide"), but that ordering only
stays safe if the intermediate it creates has somewhere to live: a value
already near the declared ceiling, multiplied by a free-standing numerator
with no relationship to that ceiling, can overflow `i64` long before the
final quotient — or either operand alone — would. Widening to `i128` first
means the multiply itself can never be the thing that silently fails; the
only overflow that can occur is the narrowing step at the end, which is
checked explicitly. A caller reasoning about whether a multiply will succeed
only needs to reason about the final result's range, never about whether the
arithmetic got there safely.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:140-145` | `mul_ratio_minor` — the widen, multiply, divide, narrow sequence |
| `src/lib.rs:153-182` | `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio` — one caller per kind |
| `src/lib.rs:217-233` | `price_mul_qty` — the fourth caller, with the quantity as the ratio |
| `../algorithm/001_widened_multiply_before_narrow.md` | The full procedure this invariant is a property of |

### Tests

| File | Relationship |
|------|--------------|
| `tests/ratio_and_div_round_test.rs` | `mul_ratio_survives_an_intermediate_that_would_overflow_i64` — an identity ratio whose product alone overflows `i64` still succeeds; `mul_ratio_reports_overflow_when_the_result_leaves_the_declared_range` — the narrowing check, under test, distinct from the intermediate the `i128` widening already absorbs |
