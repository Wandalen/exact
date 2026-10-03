# Algorithm: Rounding Division

### Scope

- **Purpose**: State exactly how `round_div` computes a rounded quotient and when it fails, so a caller can predict both the result and the overflow/zero-divisor edge cases without reading the implementation.
- **Responsibility**: `round_div` — the sign normalization, truncating divide, and mode-driven remainder adjustment it performs — and the two failure modes `RoundError` reports.
- **In Scope**: The procedure's steps, the correctness argument for each mode's adjustment, and both `RoundError` variants.
- **Out of Scope**: Which mode applies when a caller chooses none (→ [Half-Even As The Default](../decisions/001_half_even_as_the_unbiased_default.md)); why this function lives in `exact_round` rather than in its consumers (→ [`round_div` Owned By `exact_round`](../decisions/002_round_div_owned_by_exact_round.md)); the meaning of each `Rounding` variant itself (→ [Rounding Mode](../type/001_rounding_mode.md)).

### Procedure

1. Refuse a zero divisor outright — `RoundError::DivZero`.
2. Normalize a negative divisor: if `d < 0`, negate both `n` and `d`
   (`checked_neg`, surfacing `RoundError::Overflow` if either negation
   overflows — only reachable when `n` or `d` is `i64::MIN`, whose negation
   has no representable value). Every later step sees only a positive
   divisor.
3. Truncating divide: `q = n / d`, `r = n % d` (Rust's own semantics — `r`
   carries the sign of `n`).
4. If `r == 0` the division was exact; return `q` unchanged for every mode —
   no mode can disagree about a result with no remainder.
5. Otherwise adjust `q` by the chosen mode:
   - **`Down`**: if `r < 0`, subtract 1 from `q` (`checked_sub`) to round
     toward negative infinity; otherwise `q` is already the floor.
   - **`Up`**: if `r > 0`, add 1 to `q` (`checked_add`) to round toward
     positive infinity; otherwise `q` is already the ceiling.
   - **`HalfEven`**: widen `r`'s magnitude and double it
     (`r.unsigned_abs() as i128 * 2`), compare against `d` widened to
     `i128`:
     - strictly less than `d` — the remainder is under half the divisor;
       keep `q`.
     - strictly greater than `d`, or exactly equal to `d` with `q` odd —
       round away from zero (subtract 1 if `n < 0`, else add 1).
     - exactly equal to `d` with `q` already even — an exact tie landing on
       the even neighbour already; keep `q`.

   Every `checked_sub`/`checked_add` above surfaces `RoundError::Overflow` on
   failure — only reachable when the adjustment would carry `q` one past
   `i64::MIN` or `i64::MAX`.

### Why The Tie Comparison Widens To `i128`

A truncating divide's remainder always satisfies `|r| < |d|`, so doubling
`r`'s magnitude can reach just under `2 * i64::MAX` — past what `i64` holds
when `d` is itself close to `i64::MAX`. Widening both `r.unsigned_abs()` and
`d` to `i128` before doubling and comparing means the tie check itself can
never overflow, regardless of how close `n` and `d` are to the `i64` bounds
that `RoundError::Overflow` exists to guard elsewhere in this same function.

### Failure Modes

`RoundError` reports the two ways `round_div` can fail: `DivZero` (a zero
divisor) and `Overflow` (negating `i64::MIN` while normalizing a negative
divisor — the only overflow reachable; once the divisor is positive, the
quotient is at most half the range, so adjusting it by one always fits). It implements `Display` (a one-line message per variant) and
`core::error::Error`, so it composes with `?` and any call site expecting
`dyn Error`.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:82-105` | `RoundError`, its `Display` impl, and its `Error` impl |
| `src/lib.rs:107-133` | `round_div`'s doc comment, the zero-divisor refusal, and negative-divisor normalization |
| `src/lib.rs:135-140` | The truncating divide and the exact-division shortcut |
| `src/lib.rs:142-195` | Mode dispatch — `Down`, `Up`, `HalfEven` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/round_div_test.rs` | `round_div_refuses_a_zero_divisor`, `round_div_normalizes_a_negative_divisor` (refusal and normalization); `down_rounds_toward_negative_infinity`, `up_rounds_toward_positive_infinity`, `half_even_rounds_an_exact_tie_to_even`, `half_even_rounds_a_non_tie_to_the_nearest_neighbour` (mode dispatch); `an_exact_division_agrees_across_every_rounding_mode` (the exact-division shortcut) |
