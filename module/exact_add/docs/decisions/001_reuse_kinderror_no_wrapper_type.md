# ADR-001: Reuse `KindError` Directly, No Wrapper `AddError` Type

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The family's preferred fifteen-crate design lists this crate's error type as
`AddError { Overflow, ScaleMismatch, NegNotAllowed }`, mirroring the shape of
a crate that owns its own error enum. Every fallible function here, though,
already produces a result through `exact_kind`'s own `Decimal`/`Qty` methods,
which return `exact_kind::KindError`.

## Decision

Every fallible function in this crate returns `exact_kind::KindError`
directly. No `AddError` (or similarly-named wrapper) is declared.

## Alternatives Considered

### Option 1: Declare `AddError` per the preferred design, wrapping or mirroring `KindError`

Rejected because none of the preferred design's three named variants would
have a reachable construction site distinct from what `KindError` already
covers: `Overflow` already exists as `KindError::Overflow`; `ScaleMismatch`
is unreachable for the same reason `exact_kind` itself drops it — two
different `SCALE` values are two different Rust types under this family's
const-generic representation, caught at compile time, never at runtime, so no
function here could ever construct it; `NegNotAllowed` has no reachable call
site either, since the only negation this crate exposes
(`money_checked_neg`) operates on `Money`, which is already signed and never
refuses a negation. Wrapping `KindError` in a same-shaped enum with no new
reachable variant would be a type to convert through on every call, for zero
added information.

## Consequences

**Positive:**
- Callers propagating an error from this crate's functions see exactly the
  same `KindError` `exact_kind` itself would have returned — no wrapping or
  unwrapping step, and no second error type to pattern-match against.
- Nothing here hides a `ScaleMismatch`/`NegNotAllowed` dead-code path that
  tooling would otherwise have to carry.

**Negative:**
- A future function that genuinely needs an error `KindError` cannot express
  would have to either extend `KindError` itself (reaching into a sibling
  crate's type) or introduce a wrapper at that point — deferred until a
  concrete case demonstrates the need, rather than built speculatively now.

**Neutral:**
- This crate's own public error surface is entirely borrowed from
  `exact_kind`; nothing here is a judgment call about error *shape*, only
  about not inventing a second one.

## Related

- [No Panicking Variant Yet](002_no_panicking_variant_yet.md) — the sibling decision this crate's error surface also stays narrow for
- `exact_kind`'s own `KindError` — the type this decision reuses rather than wraps
