# ADR-004: An Exact Mode That Refuses A Remainder

**Date**: 2026-10-09
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

Every `Rounding` mode so far rounds: a division with a remainder always
returns a value, one grid point or the other. Some amounts must not be
rounded at all. A trade's settlement cost, `price × quantity`, that lands
between two minor units is value created or destroyed if it is rounded, in
units too small for anyone to notice until an audit sums them. The exchange
built on this family refuses such a cost today, by repeating the widened
multiply and an exact-division check by hand
(`exchange/module/exchange_types/src/lib.rs`, `notional`), outside the
division logic [ADR-002](002_round_div_owned_by_exact_round.md) keeps in one
place. `exact_ratio::price_mul_qty` already computes the same product, but
only through a mode that rounds.

## Decision

`Rounding` gains `Exact`: the division must leave no remainder. A nonzero
remainder returns `RoundError::Inexact` instead of a rounded value; an exact
division returns its quotient, as every mode does. `Exact` is one arm of
`round_div_wide`'s per-mode decision, so `round_div` and every consumer accept
it with no division logic of their own.

Each consumer maps `Inexact` into its own error type, by an explicit `match`:

- `exact_ratio` — a new `RatioError::Inexact`. `price_mul_qty( p, q,
  Rounding::Exact )` is then the refusing settlement cost, with no new function.
- `exact_snap` — a new `SnapError::OffGrid`: under `Exact`, a value not
  already on the grid is refused, not snapped.
- `exact_dust` — the existing `DustError::Remainder`, which already means
  "the split did not divide evenly"; `Exact` refuses the same fact
  `DustTo::Reject` does.

`exact_ratio`'s `mul_ratio_minor` used to map every `RoundError` to
`RatioError::Overflow` with `|_|`, which would have reported `Inexact` as an
overflow. It now goes through the same exhaustive `match` as `div_round_minor`.

## Alternatives Considered

### Option 1: A separate `price_mul_qty_exact` function

Rejected: it would repeat `price_mul_qty`'s widened multiply for one call
site, and only for `price × quantity` — a ratio multiply, a division and a
snap that must be exact would each need their own. A mode reaches all of them
through the one function that already decides how a remainder is handled.

### Option 2: Keep the exact check in each caller

Rejected: the caller repeats the widened product and the remainder test
ADR-002 placed here — the exchange's `notional` is that repetition.

### Option 3: Map `Inexact` to each consumer's existing `Overflow`

Rejected: the result is in range — it just is not exact. A caller deciding
whether to retry with a smaller amount or reject the trade outright needs to
tell the two apart, the reason `exact_ratio`'s
[ADR-001](../../../exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md)
gives each `RatioError` variant its own fix. `exact_dust` is the exception:
its `Remainder` already names this exact fact.

## Consequences

**Positive:**
- A caller that must not round states it once, as a mode, and every
  rounding-sensitive operation in the family honours it.
- The exchange's hand-rolled `notional` arithmetic can become one
  `price_mul_qty( …, Rounding::Exact )` call.

**Negative:**
- `Rounding`, `RoundError`, `RatioError` and `SnapError` each gain a variant,
  and none is `#[ non_exhaustive ]`: a downstream exhaustive `match` on any of
  them must add an arm. Nothing outside this family matches on them today —
  the exchange aliases `SnapError` but never matches on it.
- `Exact` is the one mode that can fail on an in-range division, so every
  caller passing a mode it did not choose must handle one more error.

**Neutral:**
- `HalfEven` stays the default ([ADR-001](001_half_even_as_the_unbiased_default.md));
  `Exact` is only ever chosen explicitly.
- `money_dust_remainder`/`qty_dust_remainder` now refuse under `Exact` too —
  the remainder they would report is the one `Exact` forbids.

## Related

- [ADR-002](002_round_div_owned_by_exact_round.md) — why the mode lives in `round_div_wide`, not in each consumer
- [ADR-003](003_four_more_rounding_modes.md) — the four rounding modes added before this one
- [Rounding Mode](../type/001_rounding_mode.md) — what each variant means
- [Rounding Division](../algorithm/001_rounding_division.md) — the decision each mode makes
- [Feature 008](../../../../docs/feature/008_rounding_mode_enum.md) — the three-mode requirement this extends
