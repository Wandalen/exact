# ADR-002: `round_div` Owned By `exact_round`, Not Its Consumers

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The family's preferred/target design for this crate specified only the
`Rounding` enum and its default/name functions, with no error type and no
division function. In the actual implementation, both
[`exact_ratio`](../../../exact_ratio/readme.md) and
[`exact_snap`](../../../exact_snap/readme.md) turned out to need the same
operation: divide an integer by another, applying a rounding mode to the
remainder, including the negative-divisor normalization and half-even
tie-breaking arithmetic that operation requires. Both already depend on
`exact_round` for `Rounding` itself, so the question was where that shared
division belongs now that two tier-2 consumers need it.

## Decision

`round_div` — and the `RoundError` it reports — are implemented once, here
in `exact_round`, beyond what the preferred design originally listed for
this crate. `exact_ratio` and `exact_snap` both call it rather than each
implementing their own copy.

## Alternatives Considered

### Option 1: Implement `round_div` independently in each consumer

Let `exact_ratio` and `exact_snap` each write their own rounding division.
Rejected: both would need to re-derive the same sign-normalization and
half-even tie-breaking arithmetic — the hardest part of this operation to
get right — in two places, which is the family's own anti-duplication rule
applied to exactly the kind of logic most likely to silently diverge between
the two copies.

### Option 2: A third shared crate, separate from `exact_round`

Give the division its own crate rather than adding it to `exact_round`'s
surface. Rejected: both consumers already have a dependency edge on
`exact_round` for `Rounding`, and `round_div`'s only parameter beyond the
two operands is a `Rounding` value — a new crate would add a dependency
edge neither consumer currently needs, for a function that already has a
natural owner one hop away.

## Consequences

**Positive:**
- One implementation of the division's hardest arithmetic (sign
  normalization, the `i128`-widened tie comparison, the `i64::MIN`/`i64::MAX`
  overflow checks) is verified once, in
  [Rounding Division](../algorithm/001_rounding_division.md)'s own test
  suite, and both consumers inherit that coverage without adding a
  dependency edge.

**Negative:**
- `exact_round`'s surface grew beyond its preferred/target design, which
  named only the enum and its default/name functions — a reader expecting
  this crate to hold policy data alone, not arithmetic, has to learn that
  the division primitive lives here too.

**Neutral:**
- This is the same "own it once, where every consumer already has an edge"
  reasoning the family applies elsewhere when a tier-0 root turns out to
  back more than one tier-2 operation.

## Related

- [Rounding Division](../algorithm/001_rounding_division.md) — `round_div`
  and `RoundError` themselves
- [Half-Even As The Default](001_half_even_as_the_unbiased_default.md) — the
  other preferred-design departure in this crate, decided independently
