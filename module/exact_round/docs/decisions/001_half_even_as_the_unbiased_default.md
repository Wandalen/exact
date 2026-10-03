# ADR-001: Half-Even As The Default, Departing From The Preferred Design's `Down`

**Date**: 2026-10-01
**Status**: Superseded (2026-10-03) — `rounding_default()` now returns `Down`, as the
preferred design specifies. The bias argument below still applies to any call
site that needs an unbiased result: it passes `Rounding::HalfEven` explicitly.
**Deciders**: wandalen

## Context

The family's preferred/target design for this crate specified
`rounding_default` returning `Down`. `exact_round` is net-new — the 5-crate
family this migration replaces never offered more than one implicit rounding
behaviour, so there was no real-code default to preserve, only the planned
one to reconsider against the family's actual conserved-value requirement: a
default a caller never overrides runs on every unrounded remainder in the
system, so whichever mode is chosen compounds silently over the life of the
program.

## Decision

`rounding_default()` returns `Rounding::HalfEven`, not `Down` as originally
planned.

## Alternatives Considered

### Option 1: Keep `Down`, as planned

Round every unresolved remainder toward negative infinity. Rejected: `Down`
discards the fractional remainder in the same direction on every call,
regardless of sign — a long run of roundings systematically loses value
rather than merely imprecisely representing it, which is exactly the failure
mode a conserved-value family exists to rule out.

### Option 2: `Up`

The same bias, mirrored — every unresolved remainder systematically
manufactures value instead of losing it. Rejected for the identical reason
as `Down`, in the opposite direction.

## Consequences

**Positive:**
- A call site that doesn't choose a mode explicitly gets the one mode with
  no directional bias over a long run — ties land on whichever neighbour is
  even, which is the "round up" and "round down" neighbour equally often
  across many ties, so no systematic drift accumulates.

**Negative:**
- Departs from the originally planned API behaviour — a reader who only
  knows the family's preferred/target design would expect `Down` and has to
  learn the actual default from this record or from `rounding_default`'s own
  doc comment.
- `HalfEven` costs one widened comparison (`i128`) more than `Down`/`Up`
  need, paid on every call that hits a nonzero remainder — see
  [Rounding Division](../algorithm/001_rounding_division.md) for where that
  cost is actually paid.

**Neutral:**
- `Down` and `Up` remain fully supported, non-default choices for a call
  site with a genuine directional requirement (e.g. always truncating a fee
  toward zero after sign normalization) — this decision only fixes what
  happens when a caller states no preference.

## Related

- [Rounding Mode](../type/001_rounding_mode.md) — the three variants this
  default chooses among
- [Rounding Division](../algorithm/001_rounding_division.md) — where
  `HalfEven`'s tie-breaking arithmetic is actually implemented
