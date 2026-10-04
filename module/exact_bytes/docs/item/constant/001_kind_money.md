# 001: KIND_MONEY

## Representation

The wire discriminator byte identifying a `Money`-encoded `Wire` record.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_bytes/src/lib.rs:29`

```rust
pub const KIND_MONEY : u8 = 0;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 29 | Declaration |
| `src/lib.rs` | 165 | `money_to_wire`'s encoded discriminator |
| `src/lib.rs` | 177 | `money_from_wire`'s expected-discriminator check |
| `tests/wire_roundtrip_test.rs` | 15,87,98 | Asserting the round-tripped discriminator; constructing a tampered `Wire` directly |
| `exact_arith/src/lib.rs:110` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The `Money` arm of every encode/decode function; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |
