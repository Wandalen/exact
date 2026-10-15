# ADR-002: No Panicking Variant Yet

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The family's preferred fifteen-crate design describes this crate's owned
features as including "checked/saturating/panicking variants, named" — three
calling conventions for the same arithmetic. This crate implements the first
two (`money_add`/`qty_add`/... for checked, `money_saturating_add`/
`qty_saturating_add` for saturating) but not the third.

## Decision

No panicking arithmetic entry point is declared in this crate.

## Alternatives Considered

### Option 1: Add a panicking variant now, per the preferred design's feature list

Rejected under the YAGNI check: no real consumer anywhere in this migration
calls for a panicking arithmetic entry point — every one of the five real
crates this family is drawn from is checked-only. Building one now would be
speculative work with no concrete call site to validate its naming,
behaviour, or even whether "panic" is the right failure mode for this
family's conserved values at all (a panicked transaction is a harder failure
to recover from than a returned `Err`, which is arguably the opposite of what
a financial-value crate wants by default).

## Consequences

**Positive:**
- The crate's public surface stays exactly as wide as its real, tested
  behaviour — checked and saturating, both exercised by
  `tests/checked_and_saturating_add_test.rs`.

**Negative:**
- A caller wanting `a + b` to panic on range failure (matching, say, how
  `i64`'s own `Add` operator behaves in debug builds) has no equivalent here
  and must call a checked function and `.unwrap()`/`.expect()` itself.

**Neutral:**
- The preferred design's feature list is not falsified by this gap — "named"
  variants plural does not commit to building all three before any consumer
  asks for the third.

## Related

- [Reuse `KindError` Directly, No Wrapper Type](001_reuse_kinderror_no_wrapper_type.md) — the sibling decision keeping this crate's surface narrow
