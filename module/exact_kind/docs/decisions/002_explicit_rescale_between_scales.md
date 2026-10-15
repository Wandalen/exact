# ADR-002: Crossing Scales Is An Explicit, Rounded `rescale`

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`Decimal< 2 >` and `Decimal< 6 >` are different types, so adding cents to
millionths is already a compile error. But nothing let a caller cross from one
scale to the other: feature 019 asks for an explicit, named conversion, and
none existed. A caller had to take `minor()`, multiply or divide by a power of
ten by hand, and pick a rounding without anything naming it:

| Step | Cents → millionths | Millionths → cents |
|------|-------------------|--------------------|
| Value | `12.34` | `12.345678` |
| Stored | `1234` | `12_345_678` |
| By hand | `1234 × 10_000` | `12_345_678 / 10_000` — truncates to `1234`, silently |

## Decision

- `Decimal::rescale::< TO >( rounding ) -> Result< Decimal< TO >, KindError >`.
  - **Finer** (`TO ≥ SCALE`): multiply the stored count by `pow10( TO - SCALE )`,
    checked — an overflow is `KindError::Overflow { operation : "rescale" }`.
    Exact under every mode: it only appends zeros.
  - **Coarser** (`TO < SCALE`): divide by `pow10( SCALE - TO )` through
    `exact_round::round_div`, rounding per the caller's mode. Under
    `Rounding::Exact`, a dropped nonzero digit is
    `KindError::ExcessPrecision { digits, scale : TO }`, where `digits` is the
    value's own fractional digits — `3` for `1.005`, whatever scale holds it.
  - The result goes through `Decimal::< TO >::from_minor`, so the ceiling holds
    at the new scale too.
- `Qty::rescale` delegates to `Decimal::rescale` and re-checks non-negativity
  through `Qty::from_decimal`; `PriceAt::rescale` delegates the same way
  (→ [ADR-003](003_price_generic_over_scale.md)).
- `exact_kind` gains a dependency on `exact_round`, a crate with no
  dependencies of its own, so no cycle is possible.

## Alternatives Considered

### Option 1: A `scale_convert` free function in `exact_scale`, as the source proposal names it

Rejected: `exact_scale` holds compile-time constants and knows nothing of
`Decimal`. Converting a `Decimal` there would make `exact_scale` depend on
`exact_kind` — the reverse of the family's dependency graph.

### Option 2: A conversion without a rounding mode, truncating

Rejected: a silent truncation is the bug class this family exists to prevent.
Every other lossy operation in the family — `mul_ratio`, `div_round`, `snap`,
`dust_split` — takes a `Rounding`, and `Rounding::Exact` gives the caller a
way to refuse loss outright.

### Option 3: Rounding implemented locally in `exact_kind`

Rejected: it would be a third copy of the sign-aware, tie-breaking division
`exact_round::round_div` already is, which `exact_ratio`, `exact_snap` and
`exact_dust` all share.

## Consequences

**Positive:**
- Crossing scales is one named call whose rounding is visible at the call site.
- `Rounding::Exact` refuses a conversion that would lose a digit.

**Negative:**
- A new dependency edge, `exact_kind → exact_round`. The crate doc
  ([`docs/crate/003_exact_kind.md`](../../../../docs/crate/003_exact_kind.md))
  listed only `exact_minor` and `exact_scale`; its Design status records the
  addition.

**Neutral:**
- `rescale` reuses `KindError` rather than adding a `ScaleError`. In the
  refusal, `ExcessPrecision`'s `digits` counts the digits the value has, not
  the scale it is stored at, so `1.005` held at scale 6 is refused as 3
  digits, as `parse` refuses the same text. The first version reported the
  source scale; review of #6 found the two disagree.
- The ceiling is a count of minor units, the same at every scale, so the
  largest whole amount shrinks as the scale grows: a finer rescale of a large
  value can be refused as past the ceiling.

## Related

- [ADR-003](003_price_generic_over_scale.md) — `PriceAt< SCALE >`, which carries `rescale` too
- [Feature 019](../../../../docs/feature/019_scale_convert_explicit.md) — the requirement for an explicit conversion; its Design status records this
- [`exact_round`](../../../exact_round/readme.md) — the rounded division `rescale` drives
