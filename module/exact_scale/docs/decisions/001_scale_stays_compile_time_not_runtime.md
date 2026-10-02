# ADR-001: Scale Stays Compile-Time, Not Runtime

**Date**: 2026-10-02
**Status**: Accepted
**Deciders**: wandalen

## Context

The preferred design's own [`docs/type/002_exact_scale_types.md`](../../../../docs/type/002_exact_scale_types.md) specifies a runtime `Scale(u8)` value type, alongside the power-of-ten table and the ceiling/headroom constants this crate actually built. But the family's real representation carries scale as a `Decimal<const SCALE: u32>`/`Qty<const SCALE: u32>` const generic parameter — a compile-time fact baked into the type itself, not a value a caller passes around at runtime. Building the proposed `Scale(u8)` runtime type alongside that const-generic representation would give the family two competing ways to say the same thing, one of them unused by every real call site.

## Decision

`exact_scale` holds the power-of-ten table and the ceiling/headroom constants behind the const-generic mechanism; no runtime `Scale` type is built. A scale is a type-level fact — `Decimal<6>` and `Decimal<2>` are different Rust types, caught at compile time — never a value stored in a `Scale(u8)` field. This is a deliberate departure from the preferred design's own proposed surface, not an oversight: building `Scale(u8)` now would turn this migration into an unrelated behavioural rewrite of every same-scale check across the family, trading a check the compiler already performs for free for one that would need to be performed, and could be forgotten, at runtime.

## Alternatives Considered

### Option 1: Build `Scale(u8)` as specified, alongside the const-generic representation

Ship the proposed runtime type even though nothing in the family would construct or consult it. Rejected: a type with no real caller is dead weight that a future reader would reasonably assume is load-bearing, when it is not — the actual scale-mismatch guard lives entirely in the const generic, and a parallel runtime type documenting a check nothing performs is actively misleading rather than neutral.

### Option 2: Drop the const-generic representation and use `Scale(u8)` as the real mechanism

Make scale a runtime value everywhere, matching the preferred design literally. Rejected: this is the behavioural rewrite the Decision above calls out — every one of the family's `SCALE`-mismatch guarantees (see [`exact_kind`'s Hard Problem 13 coverage](../../../exact_kind/docs/invariant/readme.md)) currently holds because the compiler refuses to build mismatched-scale code at all; moving that check to runtime would require a new `ScaleMismatch` error variant, threading a runtime scale through every arithmetic call, and re-deriving everywhere the compile-time guarantee used to hold that it still does. No concrete problem motivates that cost.

## Consequences

**Positive:**
- The scale-mismatch guarantee costs nothing at runtime and cannot be bypassed by a caller who forgets to check it — the compiler enforces it unconditionally.
- This crate's real surface (`pow10`, the ceiling/headroom constants) stays exactly as small as what the family's const-generic types actually consume.

**Negative:**
- A reader arriving from the preferred design's own type listing will not find `Scale(u8)` here, and must read this ADR (or `exact_kind`'s own docs) to learn why the const generic is the real mechanism instead.

**Neutral:**
- `ScaleError` is likewise never built — the family's one scale-level failure mode, `pow10` exceeding the backing width, stays the panicking `const fn` precedent this crate is drawn from, so there is no runtime scale error for a `Scale(u8)` type to have carried anyway.

## Related

- [Representable Range And Headroom](../non_functional_requirement/001_representable_range_and_headroom.md) — the constants this crate actually holds, and the compile-time assertion linking them
- [`exact_kind`'s own `invariant/`](../../../exact_kind/docs/invariant/readme.md) — where the const-generic scale-mismatch guarantee this decision depends on is itself documented and tested
