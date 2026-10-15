# ADR-003: Leftover Correction via Raw Minor-Unit Reconstruction

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

Under `DustTo::First`, slot `0` must absorb `leftover` on top of its plain `share`. The preferred design's own `DustError` listing declares only `{EmptyParts, Remainder, Overflow}` — no variant distinguishing "the correction made a non-negative kind go negative" from an ordinary overflow — and implementing the correction had to decide both *how* slot 0 is adjusted and what a non-negative kind does when that adjustment would take it below zero.

## Decision

Slot 0's correction (`share.checked_add(leftover)`, where `leftover` may be negative) is computed at the raw `i64` minor-unit level, inside `slot_minor` (called for each slot by `split_with`/`split_into_with`), and only converted back to a typed `Money`/`Quantity` afterward via `from_minor`. `DustError` keeps exactly the three declared variants — a `Quantity` slot that would go negative is reported as `DustError::Overflow`, with no dedicated variant.

## Alternatives Considered

### Option 1: Apply the correction through each kind's own checked arithmetic (`Quantity::checked_add`/`checked_sub`)

Build slot 0 as `share_typed.checked_add(leftover_typed)` once both are already `Money`/`Quantity` values. Rejected: `Quantity::checked_add` only ever adds a non-negative quantity — it has no way to express "add a possibly-negative amount," which is exactly what correcting an `Up`-rounded over-allocation needs (§ [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md), step 3). Reconstructing slot 0 from its adjusted *minor count* instead handles both directions — `leftover` positive or negative — uniformly, with one code path instead of a sign-dependent branch between `checked_add` and a `checked_sub` that `Quantity` does not even expose publicly for this purpose.

### Option 2: Add a dedicated `DustError` variant for the negative-quantity case

Declare something like `DustError::NegativeAdjustment` distinct from `Overflow`. Rejected on `exact_snap`'s own precedent: it makes the identical fold-everything-into-`Overflow` choice for its own `from_minor` reconstruction, and both crates reach the negative case through the same mechanism — a reconstruction that refuses an out-of-range result. Matching that precedent keeps "a reconstruction refused the result" meaning one thing family-wide, rather than `exact_dust` alone distinguishing a case `exact_snap` already folds away.

## Consequences

**Positive:**
- One leftover-correction code path handles `Money` (signed, so the correction is a plain addition either way) and `Quantity` (non-negative, so a would-be-negative correction must be refused) without a kind-specific branch in `split_with`/`split_into_with` themselves — the branch happens later, for free, inside `from_minor`'s own range check.
- Consistent with `exact_snap`'s already-established `Overflow`-folding convention, so a caller who has already learned what `Overflow` means from one crate does not need a second mental model for the other.

**Negative:**
- A caller cannot distinguish "the split's own arithmetic overflowed" from "the correction would have made a quantity negative" by matching on `DustError` alone — both surface as `Overflow`. The distinction is recoverable by the caller re-deriving it from the inputs, the same tradeoff `exact_snap` already accepted.

**Neutral:**
- `qty_dust_split_refuses_a_first_slot_that_would_go_negative_under_up_rounding` (`tests/dust_split_test.rs`) is the test this decision is verified against — it exercises exactly the `Up`-rounding-plus-`First` combination where the correction goes negative.

## Related

- [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) — step 4 and step 5, where this decision's mechanism and its consequence each live
- [Equal-Count Split Surface](002_equal_count_split_surface.md) — the companion decision about this crate's split-function surface
