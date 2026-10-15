# ADR-001: Direct `exact_round` Dependency, Not `exact_ratio`

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The preferred design lists `exact_kind, exact_ratio` as this crate's dependencies. The actual need this crate has is an integer-count, mode-driven division — divide a total by a part count, applying a `Rounding` mode to the remainder — not a rational multiplier.

## Decision

`exact_dust` depends on `exact_round` directly, for [`exact_round::round_div`], and does not depend on `exact_ratio` at all.

## Alternatives Considered

### Option 1: Depend on `exact_ratio` as the preferred design lists, and reach `round_div` through it

`exact_ratio` already depends on `exact_round` and re-exposes rounding-driven division (`money_div_round`/`qty_div_round`). Rejected: `exact_ratio`'s own public surface is built around `Ratio` (a rational multiplier — numerator over denominator) and `money_mul_ratio`/`qty_mul_ratio`/`price_mul_ratio`. None of that is used here; this crate needs exactly one capability `exact_ratio` itself only re-exposes from a dependency it already has. Taking the dependency anyway would mean compiling and versioning a whole rational-multiplier surface for a feature this crate never touches — an unused-feature dependency in the precise sense the family's own hygiene rules discourage.

### Option 2: Re-implement rounding-aware integer division locally

Write a private `round_div`-equivalent inside `exact_dust` instead of depending on either sibling. Rejected: `exact_round::round_div`'s sign-normalization and tie-breaking logic is exactly the kind of shared mechanism `exact_round` exists to own once — `exact_ratio` and `exact_snap` already depend on it for the same reason. A private copy here would be a third implementation of the same sign-handling and tie-breaking rules, which is the duplication `exact_round`'s own placement is meant to prevent.

## Consequences

**Positive:**
- `exact_dust`'s dependency edge is exactly what it uses — one tier-0 crate (`exact_kind`) and one tier-0 crate providing the rounding primitive (`exact_round`) — with nothing pulled in for a feature this crate has no call site for.
- Matches the precedent `exact_snap` already set for the identical situation (an integer-count, mode-driven division needed directly, not through `exact_ratio`).

**Negative:**
- A reader expecting this crate's dependency graph to match the preferred design's listing verbatim has to find this record to learn why it does not.

**Neutral:**
- `exact_ratio` itself is unaffected — it still depends on `exact_round` for the same primitive, just reached through its own rational-multiplier surface rather than this crate's equal-count one.

## Related

- [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) — the procedure this dependency makes possible
- [Equal-Count Split Surface](002_equal_count_split_surface.md) — the companion decision about what this crate's own split functions mean and cover
