# ADR-003: Four More Rounding Modes Beyond The Preferred Design's Three

**Date**: 2026-10-08
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

The family's preferred/target design names three rounding modes —
`Down`, `Up` and `HalfEven` ([feature 008](../../../../docs/feature/008_rounding_mode_enum.md)).
They cover the floor, the ceiling and the unbiased default, but not two
rules a caller commonly needs: truncation toward zero, and
round-to-nearest with the tie broken away from or toward zero. The seven
modes together are the familiar set Java's `BigDecimal` `RoundingMode`
offers (less its `UNNECESSARY`). Without them in `Rounding`, a caller needing
one would have to round outside this crate, repeating the sign handling and
tie detection [ADR-002](002_round_div_owned_by_exact_round.md) keeps in one
place.

## Decision

`Rounding` gains `TowardZero`, `AwayFromZero`, `HalfUp` (nearest, a tie away
from zero) and `HalfDown` (nearest, a tie toward zero), alongside `Down`,
`Up` and `HalfEven`. Each is one arm of `round_div_wide`'s decision whether
to step from the truncated quotient, so `round_div` and every consumer —
`exact_ratio`, `exact_snap`, `exact_dust` — accept them with no change of
their own. The enum keeps its name, and `rounding_default()` still returns
`HalfEven`.

## Alternatives Considered

### Option 1: Keep three modes, as planned

Rejected: a caller needing truncation or a half-up tie would round on its own,
outside `exact_round`, duplicating the division logic ADR-002 placed here to
avoid exactly that.

### Option 2: Java's names, where `Up`/`Down` mean away from/toward zero

Rejected: this crate's `Down` and `Up` already mean the floor and the ceiling.
Reusing the names for the zero-relative rules would silently change what every
existing caller's `Down` or `Up` does. The new zero-relative modes are named
for what they do — `TowardZero`, `AwayFromZero` — instead.

### Option 3: Rename the enum to `RoundingMode`

Rejected: every consumer and the facade name `Rounding`; a rename would break
each of them for no change in meaning.

### Option 4: Mark `Rounding` `#[ non_exhaustive ]`

Not adopted here: the attribute is itself a breaking change for any downstream
exhaustive `match`, and nothing in this family matches on `Rounding` outside
this crate. Left to the maintainer, as a separate change if wanted.

## Consequences

**Positive:**
- Every rounding-sensitive operation in the family accepts the four modes
  without a code change of its own.
- The rounding rules stay in one function — one `match` arm per mode.

**Negative:**
- A downstream exhaustive `match` on `Rounding` must handle four more arms.
- `HalfUp` takes `-2.5` to `-3`, down, despite its name — the common
  convention, stated in the variant's doc comment.
- Feature 008's Statement still names three modes until the maintainer
  approves extending it.

**Neutral:**
- `HalfEven` stays the default and the only mode with no directional bias:
  `TowardZero` and `HalfDown` lean toward zero, `AwayFromZero` and `HalfUp`
  away from it. [ADR-001](001_half_even_as_the_unbiased_default.md) is
  unchanged.
- The tie comparison the three `Half*` modes share (twice `|r|` against
  `|d|`, in `u128`) is computed once, before the per-mode decision.

## Related

- [ADR-001](001_half_even_as_the_unbiased_default.md) — the default these modes do not change
- [ADR-002](002_round_div_owned_by_exact_round.md) — why the modes live in `round_div_wide`, not in each consumer
- [Rounding Mode](../type/001_rounding_mode.md) — what each of the seven variants means
- [Rounding Division](../algorithm/001_rounding_division.md) — the decision each mode makes
- [Feature 008](../../../../docs/feature/008_rounding_mode_enum.md) — the three-mode requirement this extends
