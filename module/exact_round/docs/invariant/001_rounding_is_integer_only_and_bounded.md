# Invariant: Rounding Is Integer-Only And Bounded

### Scope

- **Purpose**: State that `round_div` never touches a float and never diverges from the true quotient by more than one unit, so a caller can treat its result as a deterministic, exactly-reasoned rounding rather than one subject to platform float behavior.
- **Responsibility**: `round_div`.
- **In Scope**: The arithmetic types `round_div` computes with, and the bound on how far its result can sit from the unrounded quotient.
- **Out of Scope**: Why `HalfEven` specifically is the unbiased default (→ `../decisions/001_half_even_as_the_unbiased_default.md`, which owns the bias argument); the tie-breaking procedure itself (→ `../algorithm/001_rounding_division.md`).

### Statement

`round_div` (`src/lib.rs:109-187`) computes exclusively in `i64`/`i128`
integer arithmetic — no `f32`/`f64` appears anywhere in its body, including
the `HalfEven` tie-detection branch, which widens to `i128` via an `as` cast
rather than a float comparison (`src/lib.rs:161-163`). Its result never
differs from the true, infinite-precision quotient `n / d` by one unit or
more: every branch either returns the truncated quotient `q` unchanged or
adjusts it by exactly `±1` (`checked_add`/`checked_sub` by `1`, never by any
other amount), and only when the remainder is nonzero.

### Rationale

This family's reason for existing is that a float's rounding is not
deterministic in the way an exact, conserved value needs — the same division
can observably differ across platforms, compiler versions, or optimization
levels when a float intermediate is involved. `round_div` sidesteps that
entirely by never constructing one: the quotient and remainder come from
`i64`'s own integer division, and the one place a wider type is needed to
detect an exact half-way tie (comparing `2 * |r|` against `d` without
overflowing) uses an `i128` integer widening, not a float ratio. Bounding the
adjustment to exactly one unit is what makes "rounding" the right word for
what this function does rather than an arbitrary remapping — every result is
either the truncated quotient or its immediate neighbor, never further.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:109-132` | `round_div`'s divisor normalization and the truncated quotient/remainder it rounds from |
| `src/lib.rs:133-186` | The three `Rounding` branches, each adjusting by at most one unit |
| `src/lib.rs:159-185` | `HalfEven`'s tie detection — `i128`-widened integer comparison, no float |

### Tests

| File | Relationship |
|------|--------------|
| `tests/round_div_test.rs` | Every rounding mode at both signs, plus the `i64::MIN` overflow boundary where the one-unit adjustment itself cannot be represented |
| `tests/rounding_mode_test.rs` | `Rounding`'s own variant coverage and stable-name accessor |
