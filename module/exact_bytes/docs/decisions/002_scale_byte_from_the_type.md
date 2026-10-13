# ADR-002: A Record's Scale Byte Is The Type's Own Scale

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

Every record carried one scale byte, `SCALE_BYTE`, a cast of
`exact_scale::MONEY_SCALE` guarded by a compile-time assertion that it fits a
`u8`. With every operation generic over the scale
(→ [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)),
a `Decimal< 2 >` written with that byte would claim to be six places, and a
reader would take `12.34` — stored `1234` — as `0.001234`.

## Decision

- A private `const fn scale_byte< const S : u32 >() -> u8` returns `S` as the
  record's scale byte. An inline `const { assert!( S <= u8::MAX as u32 ) }`
  fails the build for any scale the cast would cut, once per scale used — the
  same guarantee the old assertion gave for `MONEY_SCALE`.
- `*_to_wire::< S >` writes `scale_byte::< S >()`. `*_from_wire::< S >` refuses
  a record whose scale byte is not `S` with `WireError::BadScale` — checked
  after the kind byte, as before.
- `SCALE_BYTE` and its assertion are removed. `exact_scale` is no longer used by
  this crate's own code, and moves to `[dev-dependencies]` for the test that
  decodes at `MONEY_SCALE`.

## Alternatives Considered

### Option 1: Keep one fixed scale byte

Rejected: a record would misstate its own scale, and decoding it would be off
by a power of ten with nothing to catch it.

### Option 2: Decode at whatever scale the record carries, converting

Rejected: a conversion can round, and a decoder that rounds hides it. A caller
decodes at the writer's scale and rescales explicitly
(→ [`exact_kind`'s ADR-002](../../../exact_kind/docs/decisions/002_explicit_rescale_between_scales.md)).

## Consequences

**Positive:**
- A record states its real scale, and a reader at another scale is refused, not
  misled.
- The record layout is unchanged; a value at the standard scale is still
  written with scale byte `6`, so existing records decode as before.

**Negative:**
- `*_from_wire` takes its scale only from its return type, so a call where
  nothing else fixes it needs `::< MONEY_SCALE >` — recorded in
  `exact_arith`'s ADR-001.

**Neutral:**
- The crate depends on `exact_kind` alone at run time.

## Related

- [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md) — the family-wide change
- [Scale Byte Never Truncates](../invariant/001_scale_byte_never_truncates.md) — the invariant, now held by `scale_byte`
- [Wire Record Encoding](../format/001_wire_record_encoding.md) — the record layout, unchanged
