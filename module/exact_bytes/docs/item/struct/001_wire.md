# 001: Wire

## Representation

A fixed-width wire encoding for one conserved value: its minor-unit count,
the scale it was written at, and which kind it is — a self-describing
10-byte record, wider than the preferred design's bare 8-byte `amount`
field, because a bare amount cannot be decoded back into a specific kind
without an external convention recording which kind and scale it was
written at (module doc comment, `src/lib.rs:7-13`).

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_bytes/src/lib.rs:86-92`

```rust
/// A fixed-width wire encoding for one conserved value: its minor-unit
/// count, the scale it was written at, and which kind it is.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Wire
{
  minor : i64,
  scale : u8,
  kind : u8,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 87-92 | Declaration |
| `src/lib.rs` | 94-159 | `impl Wire` — all 6 methods plus `ENCODED_LEN` |
| `src/lib.rs` | 165,175,192,203,220,228 | Constructed or taken as a parameter by all 6 to/from-wire functions |
| `tests/wire_roundtrip_test.rs` | 14-98 (throughout) | Constructed via every to-wire function and via `Wire::new`/`Wire::from_bytes` directly |
| `exact_arith/src/lib.rs:113` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:41` | — | Constructed via `money_to_wire` in the facade's own end-to-end test |

All 3 fields are private — readable only through [`minor`](../associated_function/003_wire_minor.md)/[`scale`](../associated_function/004_wire_scale.md)/[`kind`](../associated_function/005_wire_kind.md), or directly within `exact_bytes` itself (every `*_from_wire` function reads `w.minor`/`w.scale`/`w.kind` as plain field accesses rather than through the accessor methods — see those methods' own Caller Trees for the resulting "accessor exists, never called" finding on two of the three).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The one wire-format record type; exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-exported, and actually constructed in the facade's own Money round-trip test — unlike most of this crate's other items, which the facade only re-exports without exercising |
