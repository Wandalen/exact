# ADR-002: Equal-Count Split Surface — `usize` Parts, No `price_dust_split`

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The preferred design's signature, `dust_split(total, parts, mode, to)`, does not say what `parts` means — an equal-share count, or a list of weights, one per share. It also does not say which conserved kinds get a `dust_split` function at all.

## Decision

`parts` is a plain `usize` — the number of equal shares to divide `total` into — not a weighted list. Split functions are shipped for `Money` and `Quantity` only; there is no `price_dust_split`.

## Alternatives Considered

### Option 1: `parts` as a weighted list (`&[u32]` or similar), splitting by ratio

Accept a list of per-share weights and divide in proportion to each, the shape the family's older, now-deleted weighted-split design sketched. Rejected: nothing in this crate's own name or its callers needs proportional splitting — "dust" is this family's plain, already-documented term for the remainder of an *equal* integer division, and a weighted-list parameter would silently redefine that meaning while adding a combinatorial surface (which weights, how many, what happens when they sum to zero) this crate has no present caller for. If a genuine proportional-split need appears, it is a different, explicitly-named operation, not an overload of `dust_split`.

### Option 2: Also ship `price_dust_split`, for symmetry with `Money`/`Quantity`

Rejected on `exact_ratio`'s own precedent: it ships `money_div_round`/`qty_div_round` with no `price_div_round`, because "splitting a total among parties" and "splitting a holding into lots" are real operations for `Money` and `Quantity`, while splitting a *price* into equal shares has no natural reading and no consumer anywhere in this codebase. The same reasoning applies here without modification — a price is a rate, not a holding, and dividing a rate "into parts" does not correspond to anything a caller has ever asked for.

## Consequences

**Positive:**
- The parameter name `parts` and its type (`usize`) match the family's other count-driven operations, so a reader already familiar with `exact_round`/`exact_snap` needs no new mental model for what this crate's `parts` means.
- The two shipped functions (`money_dust_split`, `qty_dust_split`) have exactly one job each, with no weighted-list edge cases (empty list, zero-sum weights, mismatched lengths) to specify or test.

**Negative:**
- A caller who actually needs a proportional, weight-driven split has nothing to reach for in this crate — the gap is deliberate, not an oversight, but it is still a gap relative to the older weighted-split design sketch.

**Neutral:**
- Adding `price_dust_split` later, if a real use ever names one, would be a pure addition — nothing about the current `Money`/`Quantity` surface would need to change to accommodate it.

## Related

- [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) — the procedure this surface decision shapes
- [Direct `exact_round` Dependency, Not `exact_ratio`](001_direct_exact_round_dependency.md) — the companion decision about how the per-share division itself is reached
