# Invariant: Rounding Is Integer-Only And Bounded

### Scope

- **Purpose**: State that `round_div` never touches a float and never diverges from the true quotient by more than one unit, so a caller can treat its result as a deterministic, exactly-reasoned rounding rather than one subject to platform float behavior.
- **Responsibility**: `round_div`, and `round_div_wide`, which holds the rounding rules for both.
- **In Scope**: The arithmetic types `round_div` computes with, and the bound on how far its result can sit from the unrounded quotient.
- **Out of Scope**: Which mode is the default, and its history (→ `../decisions/001_half_even_as_the_unbiased_default.md`, which owns the bias argument); the tie-breaking procedure itself (→ `../algorithm/001_rounding_division.md`).

### Statement

`round_div` (`src/lib.rs:118-126`) and `round_div_wide`
(`src/lib.rs:140-175`) compute exclusively in `i64`/`i128`/`u128` integer
arithmetic — no `f32`/`f64` appears anywhere in either body, including the
`HalfEven` tie-detection branch, which compares doubled magnitudes as `u128`
integers rather than a float ratio (`src/lib.rs:161-167`). The result never
differs from the true, infinite-precision quotient `n / d` by one unit or
more: it is either the truncated quotient `q` unchanged or `q` moved by
exactly one (`q - 1` or `q + 1`, never any other amount), and only when the
remainder is nonzero.

### Rationale

This family's reason for existing is that a float's rounding is not
deterministic in the way an exact, conserved value needs — the same division
can observably differ across platforms, compiler versions, or optimization
levels when a float intermediate is involved. `round_div` sidesteps that
entirely by never constructing one: the quotient and remainder come from
integer division, and the one place a wider type is needed to detect an
exact half-way tie (comparing `2 * |r|` against `|d|` without overflowing)
uses a `u128` integer comparison, not a float ratio. Bounding the
adjustment to exactly one unit is what makes "rounding" the right word for
what this function does rather than an arbitrary remapping — every result is
either the truncated quotient or its immediate neighbor, never further.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:118-152` | `round_div`'s widening into `round_div_wide`, and the truncated quotient/remainder it rounds from |
| `src/lib.rs:154-174` | The three `Rounding` branches deciding a step, and the step itself — at most one unit |
| `src/lib.rs:161-167` | `HalfEven`'s tie detection — a `u128` integer comparison, no float |

### Tests

| File | Relationship |
|------|--------------|
| `tests/round_div_test.rs` | Every rounding mode at both signs, each mode against its definition on a grid, and the minimum value as either operand, where only `MIN / -1` overflows |
| `tests/rounding_mode_test.rs` | `Rounding`'s own variant coverage and stable-name accessor |
