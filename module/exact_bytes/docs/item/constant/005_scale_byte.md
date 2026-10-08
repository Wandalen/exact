# 005: SCALE_BYTE

## Representation

The scale every kind is written at, as the single byte a `Wire` record stores
in its `scale` field (`src/lib.rs:123`). It holds the crate's one
`exact_scale::MONEY_SCALE as u8` cast, so the three `*_to_wire` functions
write it and `check_header` compares a decoded record against it, rather than
each repeating the cast. The cast cannot truncate: the compile-time assertion
directly above it, [`const _`](004_money_scale_fits_u8_assertion.md), fails
the build first if `MONEY_SCALE` ever outgrows a byte.

Private — not re-exported by `exact_arith`, and not part of this crate's
public surface.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_bytes/src/lib.rs:52`

```rust
const SCALE_BYTE : u8 = exact_scale::MONEY_SCALE as u8;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 52 | Declaration |
| `src/lib.rs` | 110 | `check_header`'s scale check, shared by the three `*_from_wire` functions |
| `src/lib.rs` | 198, 218, 239 | The `scale` byte `money_to_wire`, `qty_to_wire` and `price_to_wire` write |

No test names it directly — it is private; `tests/wire_roundtrip_test.rs`
checks its value through a record (`an_encoded_record_carries_minor_scale_and_kind`
asserts a `scale` of `6`) and its check through `a_mismatched_scale_byte_is_refused`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The one wire scale byte, written by every encoder and checked by every decoder |
