# ADR-001: No CmpError, Unconditional Ord

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The preferred design names `CmpError { ScaleMismatch }` and warns against
implementing `Ord` unconditionally "where scales may differ without a
documented same-scale invariant." That warning is written for a
representation where scale is a runtime value two operands being compared
could disagree on — which is not this family's representation, and the
crate had to decide whether to build the guard anyway or rely on what the
type system already guarantees.

## Decision

`exact_cmp` defines no `CmpError` and no conditional `Ord`. Every function —
`money_cmp`, `qty_cmp`, `price_cmp`, `money_eq`, `price_min`, `price_max` —
is infallible, dispatching directly to `exact_kind::Decimal`/`Qty`'s own
derived `Ord`/`PartialEq`.

Under this family's `Decimal<const SCALE: u32>`, `Decimal<6>` and
`Decimal<9>` are different Rust types. Two values of different scales cannot
reach `money_cmp` (or any function here) to be compared at all — the
compiler refuses the call before this crate's code runs — so there is no
runtime state `CmpError::ScaleMismatch` could ever report, and no "documented
same-scale invariant" for `Ord` to be conditioned on beyond the one the type
system already enforces.

## Alternatives Considered

### Option 1: Declare `CmpError { ScaleMismatch }` and a fallible `try_cmp`

Build the guard the preferred design describes, even though no call site
could ever produce `ScaleMismatch`. Rejected: an error variant and a
`Result`-returning API with no reachable failure path is a correctness
overhead with no correctness benefit — every caller would handle a `Result`
that is provably always `Ok`, and the one compile-time guarantee this family
already has would be hidden behind a runtime-shaped API that doesn't need it.

### Option 2: A marker type or trait bound asserting same-scale, checked at compile time

Encode the same-scale requirement as an explicit bound rather than leaving it
implicit in the function signature's shared `SCALE` parameter. Rejected as
unneeded ceremony: `money_cmp(a: Money, b: Money)` already requires both
arguments to be the same concrete type, which already requires the same
`SCALE` — there is no additional fact left to assert.

## Consequences

**Positive:**
- Every comparison and extremum function here is a direct, zero-cost
  dispatch to a derived trait method — no error type to construct, match on,
  or propagate for a failure that cannot occur.
- `price_min`/`price_max` can return a bare `Price` rather than
  `Result<Price, CmpError>`, which is what makes them usable at a call site
  expecting a plain value.

**Negative:**
- A caller porting code written against the preferred design's fallible
  `CmpError`-returning signatures must drop the `Result` handling entirely
  rather than finding it merely simplified.

**Neutral:**
- The same reasoning — an error variant with no reachable construction site
  under a const-generic scale — already dropped `ScaleMismatch` from
  `exact_kind::KindError` and from
  [`exact_ratio`'s own `RatioError`](../../../exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md);
  this is the same judgment call applied to comparison rather than
  arithmetic.

## Related

- [`exact_kind`](../../../exact_kind/readme.md) — the derived `Ord`/`PartialEq` every function here dispatches to
- [`exact_ratio`'s own deviation](../../../exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) — the same compile-time-scale reasoning applied to arithmetic instead of comparison
