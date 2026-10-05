# 001: Wire::ENCODED_LEN

## Representation

The encoded length in bytes: an 8-byte little-endian `minor`, one `scale`
byte, and one `kind` byte.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_bytes/src/lib.rs:111`

```rust
pub const ENCODED_LEN : usize = 10;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 111 | Declaration |
| `src/lib.rs` | 148,164 | Array size in `to_bytes`'s return type; slice-length check in `from_bytes` |
| `tests/wire_roundtrip_test.rs:67` | — | Constructing a slice one byte shorter than the encoded length, to exercise `WireError::Truncated` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The fixed width both `to_bytes`/`from_bytes` agree on; exercised by its own truncation test |
