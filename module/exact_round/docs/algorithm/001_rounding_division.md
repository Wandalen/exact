# Algorithm: Rounding Division

### Scope

- **Purpose**: State exactly how `round_div` and `round_div_wide` compute a rounded quotient and when they fail, so a caller can predict both the result and the overflow/zero-divisor edge cases without reading the implementation.
- **Responsibility**: `round_div_wide` — the truncating divide and the mode-driven step toward the exact quotient — and `round_div`, its `i64` form; and the three failure modes `RoundError` reports.
- **In Scope**: The procedure's steps, the correctness argument for each mode's adjustment, and all three `RoundError` variants.
- **Out of Scope**: Which mode applies when a caller chooses none (→ [Half-Even As The Default](../decisions/001_half_even_as_the_unbiased_default.md)); why this function lives in `exact_round` rather than in its consumers (→ [`round_div` Owned By `exact_round`](../decisions/002_round_div_owned_by_exact_round.md)); the meaning of each `Rounding` variant itself (→ [Rounding Mode](../type/001_rounding_mode.md)).

### Procedure

`round_div` widens both `i64` operands to `i128`, runs the steps below
through `round_div_wide`, and narrows the result back — reporting
`RoundError::Overflow` when it does not fit an `i64`, which only
`i64::MIN / -1` reaches. `round_div_wide` holds the rounding rules, once:

1. Refuse a zero divisor outright — `RoundError::DivZero`.
2. Truncating divide, with the operands' own signs: `q = n / d` (via
   `checked_div`, surfacing `RoundError::Overflow` only for
   `i128::MIN / -1`, the one quotient with no representable value) and
   `r = n % d` (Rust's own semantics — `r` carries the sign of `n`).
3. If `r == 0` the division was exact; return `q` unchanged for every mode —
   no mode can disagree about a result with no remainder.
4. Otherwise the exact quotient lies strictly between `q` and its neighbour
   one step further from zero: below `q` when `r` and `d` differ in sign,
   above it otherwise.
5. Decide, by the chosen mode, whether to step toward the exact quotient:
   - **`Down`**: step when it is below `q` — the floor.
   - **`Up`**: step when it is above `q` — the ceiling.
   - **`HalfEven`**: compare twice `|r|` with `|d|`, both as `u128`:
     - less — the remainder is under half the divisor; keep `q`.
     - greater, or equal with `q` odd — step.
     - equal with `q` already even — an exact tie landing on the even
       neighbour already; keep `q`.
   - **`TowardZero`**: never step — `q` is already truncated toward zero.
   - **`AwayFromZero`**: always step — the neighbour is the one further
     from zero.
   - **`HalfUp`**: compare twice `|r|` with `|d|` as `HalfEven` does; step
     when it is greater or equal — an exact tie goes away from zero.
   - **`HalfDown`**: compare twice `|r|` with `|d|` as `HalfEven` does; step
     only when it is greater — an exact tie stays at `q`, toward zero.
   - **`Exact`**: neither keep nor step — any remainder is refused as
     `RoundError::Inexact`, so step 6 is never reached.
6. Step: `q - 1` when the exact quotient is below `q`, `q + 1` otherwise.
   This never overflows: a nonzero remainder needs `|d| >= 2`, so
   `|q| <= |n| / 2`.

### Why The Operands Keep Their Signs

Making the divisor positive first — negating both operands when `d < 0` —
would let every later step see one sign case, but negating the type's
minimum value has no representable result. `0 / MIN`, `1 / MIN` and
`MIN / -2` would then fail although each quotient fits. Comparing the signs
of `r` and `d` (step 4) tells the same direction without negating anything,
so a minimum-value operand rounds like any other.

### Why The Tie Comparison Uses `u128`

A truncating divide's remainder always satisfies `|r| < |d| <= 2^127`, so
twice `r`'s magnitude can reach just under `2^128` — past what `i128` holds,
but within `u128`. Taking `unsigned_abs()` of both and doubling in `u128`
means the tie check itself can never overflow.

### Failure Modes

`RoundError` reports the three ways a rounded division can fail: `DivZero` (a
zero divisor), `Overflow` (the quotient does not fit the integer type —
only `MIN / -1`, for `round_div` at `i64` and `round_div_wide` at `i128`), and
`Inexact` (`Rounding::Exact` was asked and the division left a remainder —
step 5).
It implements `Display` (a one-line message per variant) and
`core::error::Error`, so it composes with `?` and any call site expecting
`dyn Error`.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:108-134` | `RoundError`, its `Display` impl, and its `Error` impl |
| `src/lib.rs:136-155` | `round_div` — widen, divide through `round_div_wide`, narrow back |
| `src/lib.rs:157-176` | `round_div_wide`'s doc comment and the zero-divisor refusal |
| `src/lib.rs:186-192` | The truncating divide and the exact-division shortcut |
| `src/lib.rs:194-219` | The direction of the exact quotient, the mode decision for each of the eight modes, and the step |

### Tests

| File | Relationship |
|------|--------------|
| `tests/round_div_test.rs` | `round_div_refuses_a_zero_divisor`, `a_negative_divisor_rounds_like_negating_both_operands` (refusal and sign handling); `down_rounds_toward_negative_infinity`, `up_rounds_toward_positive_infinity`, `half_even_rounds_an_exact_tie_to_even`, `half_even_rounds_a_non_tie_to_the_nearest_neighbour` (mode dispatch); `an_exact_division_agrees_across_every_rounding_mode` (the exact-division shortcut); `every_mode_matches_its_definition_on_a_grid` (every mode against its definition); `every_mode_rounds_as_named` (every mode against hand-worked values, ties and non-ties at both signs); `round_div_reports_overflow_only_when_the_quotient_does_not_fit`, `round_div_handles_the_minimum_value_on_either_side`, `round_div_wide_handles_the_minimum_value_on_either_side` (the minimum value and the one overflow) |
