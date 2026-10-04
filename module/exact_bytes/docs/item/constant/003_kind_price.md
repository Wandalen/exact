# 003: KIND_PRICE

## Representation

The wire discriminator byte identifying a `Price`-encoded `Wire` record.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_bytes/src/lib.rs:33`

```rust
pub const KIND_PRICE : u8 = 2;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 33 | Declaration |
| `src/lib.rs` | 220 | `price_to_wire`'s encoded discriminator |
| `src/lib.rs` | 230 | `price_from_wire`'s expected-discriminator check |
| `tests/wire_roundtrip_test.rs:35` | — | Asserting the round-tripped discriminator |
| `exact_arith/src/lib.rs:111` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The `Price` arm of every encode/decode function; exercised by its own test |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |
