# 001: KIND_MONEY

## Representation

The wire discriminator byte identifying a `Money`-encoded `Wire` record.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_bytes/src/lib.rs:42`

```rust
pub const KIND_MONEY : u8 = 0;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 42 | Declaration |
| `src/lib.rs` | 198 | `money_to_wire`'s encoded discriminator |
| `src/lib.rs` | 210 | `money_from_wire`'s expected discriminator, passed to `check_header` |
| `tests/wire_roundtrip_test.rs` | 15,87,98 | Asserting the round-tripped discriminator; constructing a tampered `Wire` directly |
| `exact_arith/src/lib.rs:122` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The `Money` arm of every encode/decode function; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |
