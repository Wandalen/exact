# ADR-001: Wire Error Adds Overflow And Negative

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The preferred design names `WireError { BadKind, BadScale, Truncated }`.
`Wire::new` is infallible and its `minor` field is a bare `i64` with no range
check of its own — the range check happens only when a `*_from_wire`
function turns a decoded record into a specific kind. None of the preferred
design's three variants covers what that check can find: a `minor` that
decodes to a perfectly valid `i64` but still breaches the kind's declared
ceiling, or, for `Quantity`, a `minor` below zero.

## Decision

`WireError` carries five variants: the preferred design's `BadKind`,
`BadScale`, and `Truncated`, plus `Overflow` and `Negative { minor }`.

Both additions are genuinely reachable, unlike the unreachable variants this
family drops elsewhere: a `Wire` can be built directly via `Wire::new` (or
decoded from attacker- or corruption-controlled bytes via `from_bytes`) with
any `i64` in its `minor` field, including one past
`exact_kind::Decimal::MAX`/`MIN` or, for a `Quantity`, a negative one. Both
cases are exercised directly:
`a_decoded_value_past_the_ceiling_is_refused_as_overflow` builds a `Wire` one
past `Money::MAX` and expects `WireError::Overflow`;
`a_decoded_negative_value_is_refused_only_for_a_non_negative_kind` builds a
`Wire` with `minor: -1` and expects `WireError::Negative` from the `Quantity`
decoder while the same bytes decode successfully as `Money`.

## Alternatives Considered

### Option 1: Fold both into `BadScale` or a generic decode failure

Report an out-of-range or negative `minor` as a repurposed existing variant
rather than adding new ones. Rejected: the family's own general principle,
stated in `exact_kind`'s module doc, is that a generic failure moves the
investigation to the debugger — reporting a ceiling breach as `BadScale`
would send a caller looking at the wrong field entirely.

### Option 2: Make `Wire::new` fallible instead, checking the range at construction

Give `Wire::new` the range check instead of deferring it to the
`*_from_wire` functions. Rejected: `Wire` alone does not know which kind it
will become, and a kind's ceiling and non-negativity rule are properties of
the kind, not of the wire record — `Decimal`'s own `MAX`/`MIN` already live
in `exact_kind`. Checking at `Wire::new` would mean either duplicating that
range here or taking a dependency edge back onto a specific kind, which
`Wire` — a single record shared by three kinds — cannot do for all three at
once.

## Consequences

**Positive:**
- A decoded record that is wrong in a way `Wire` itself did not anticipate —
  tampered bytes, a stale writer, a truncated-then-padded file — is reported
  precisely rather than silently accepted or mis-attributed to the wrong
  check.
- `Overflow` and `Negative` reuse the same two-variant shape this family
  already uses for the identical reason in `exact_kind::KindError` and
  [`exact_ratio`'s `RatioError`](../../../exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) —
  one vocabulary for "the arithmetic succeeded but the kind refuses the
  result," reused rather than reinvented a third time.

**Negative:**
- A caller exhaustively matching on the preferred design's three-variant
  `WireError` must add two arms rather than finding the type unchanged.

**Neutral:**
- `Wire::new` itself stays infallible — these two variants are returned only
  by `money_from_wire`/`qty_from_wire`/`price_from_wire`, never by `Wire`'s
  own constructor or accessors.

## Related

- [Wire Record Encoding](../format/001_wire_record_encoding.md) — the record whose `minor` field this decision's two variants guard
- [`exact_ratio`'s own `Negative` addition](../../../exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) — the same addition, for the same reason, in a sibling tier-2 crate
