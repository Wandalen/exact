# ADR-004: A Weighted Split Is Its Own, Named Operation

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

[ADR-002](002_equal_count_split_surface.md) made `parts` a plain equal-share
count and rejected a weighted list *as an overload of `dust_split`*, because
nothing needed one yet. It also said what to do if that changed: "If a
genuine proportional-split need appears, it is a different,
explicitly-named operation, not an overload of `dust_split`."

The need has appeared. The exchange's fees (X8) split one charge between
several receivers by percentage — 70 % to the exchange, 20 % to a referrer,
10 % to an insurance fund — and E4 asks for that split. Rounding each share on
its own loses or invents cents: `$10.00` split three ways is `3.33 × 3 = 9.99`.

## Decision

- A new function, `money_dust_split_weighted( total, weights : &[ u64 ], mode, to )`,
  beside `money_dust_split`, which is unchanged.
- Slot `i` gets `total × weights[ i ] / Σ weights`, rounded under `mode`
  through `exact_round::round_div_wide`. The leftover — `total − Σ shares` —
  goes where `to` says, through the same `slot_minor` the equal split uses, so
  `First`, `Sink` and `Reject` mean exactly what they already mean.
- Weights are `u64` whole numbers — percentages, basis points, or amounts in
  minor units, never floats. An amount is the weight a fee split by notional
  needs: each fill's worth, up to the ceiling (about 9 × 10¹⁵), far past
  `u32::MAX` (about 4.3 × 10⁹, `$4 294.97` at scale 6). `total × weight` is
  computed in `i128` — an `i64` times a `u64` stays inside it, so it cannot
  overflow — and the weights are summed in `i128`, checked.
- Edge cases:

  | Input | Result |
  |-------|--------|
  | no weights, or every weight `0` | `DustError::EmptyParts` |
  | a zero weight | a zero share — though `DustTo::First` puts the dust on slot 0 whatever its weight |
  | `Rounding::Exact` and a share that is not whole | `DustError::Remainder` |
  | `DustTo::Reject` and a nonzero leftover | `DustError::Remainder` |
  | `DustTo::Sink` | the dust is `total` minus the sum of the parts |
  | equal weights | exactly what `money_dust_split` returns, under every mode |

- Money only: no `qty_`, `_into` or `_remainder` variant until a caller needs
  one.

## Alternatives Considered

### Option 1: Overload `money_dust_split` with weights

Rejected by ADR-002 itself: "dust" keeps its documented meaning — the
remainder of an equal division — and the equal split keeps its simple,
count-driven surface.

### Option 2: A new `DustError::ZeroWeights` variant

Clearer, but adding a variant breaks every caller that matches `DustError`
exhaustively. No weight to split by is the same fact as no part to split
into, so `EmptyParts` reports it.

### Option 3: Give the dust to the largest fractional share (largest-remainder method)

Fairer per slot, but a new rule with its own tie-break order, and different
from where the equal split sends dust. Rejected for now: `DustTo::First` is
explicit and already understood; a largest-remainder destination can be added
as a new `DustTo` variant if a fee policy asks for it.

### Option 4: `qty_`, `_into` and `_remainder` variants for symmetry

Rejected under YAGNI: E4 and X8 are about money. Each can be added beside this
one without changing it.

### Option 5: `u32` weights

Enough for percentages and basis points, and the first version took them.
Rejected in review (#7, P7-8): the split the exchange most likely needs spreads
one fee over several fills by what each was worth, and any fill past
`$4 294.97` at scale 6 does not fit a `u32`. Widening is a signature change —
free before the first release, breaking after it.

## Consequences

**Positive:**
- A fee can be split by percentage with every cent accounted for: the parts
  plus any held-back dust always equal the total, checked over 70 035
  cases — 2 001 totals × 5 weight lists × 7 rounding modes — in
  `tests/weighted_split_test.rs`.

**Negative:**
- `DustTo::First` can give the dust to a slot whose weight is zero. The
  caller orders the weights so the slot that should absorb dust comes first.

**Neutral:**
- E4 does not unblock X8 alone: fees also need a decided rate and schedule.
- No new dependency: `round_div_wide` is already in `exact_round`.

## Related

- [ADR-002](002_equal_count_split_surface.md) — the equal-count surface, and the rule this follows
- [Weighted-Parts Dust Split](../algorithm/002_weighted_parts_dust_split.md) — the procedure
- [money_dust_split_weighted](../item/function/007_money_dust_split_weighted.md) — the item
- [Leftover Is Never Silently Dropped](../invariant/001_leftover_is_never_silently_dropped.md) — the invariant, now for both splits
