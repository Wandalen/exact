# 002: KIND_QTY

## Representation

The wire discriminator byte identifying a `Quantity`-encoded `Wire` record.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_bytes/src/lib.rs:44`

```rust
pub const KIND_QTY : u8 = 1;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 44 | Declaration |
| `src/lib.rs` | 205 | `qty_to_wire`'s encoded discriminator |
| `src/lib.rs` | 218 | `qty_from_wire`'s expected-discriminator check |
| `tests/wire_roundtrip_test.rs` | 25,95 | Asserting the round-tripped discriminator; constructing a negative-value `Wire` directly |
| `exact_arith/src/lib.rs:124` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The `Quantity` arm of every encode/decode function; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |
