# 007: Wire::from_bytes

## Representation

Decode from a byte slice, refusing one shorter than
[`ENCODED_LEN`](../associated_constant/001_wire_encoded_len.md).

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:162-171`

```rust
pub fn from_bytes( bytes : &[ u8 ] ) -> Result< Self, WireError >
{
  let Some( encoded ) = bytes.get( 0 .. Self::ENCODED_LEN )
  else
  {
    return Err( WireError::Truncated );
  };
  let minor = i64::from_le_bytes( encoded[ 0 .. 8 ].try_into().expect( "exactly 8 bytes" ) );
  Ok( Self { minor, scale : encoded[ 8 ], kind : encoded[ 9 ] } )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 162-171 | Declaration |
| `tests/wire_roundtrip_test.rs` | 57,67,77,145,149 | Decoding a tampered record, a truncated slice, a raw-byte round-trip, an empty slice, and a longer slice |

No production function in `exact_bytes` or `exact_arith` calls `from_bytes`
— only test code does; every `*_from_wire` function takes an already-decoded
`Wire` value, never a raw byte slice.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `tests/wire_roundtrip_test.rs` | Exercised by 4 of the crate's 14 tests |

## Caller Tree

No caller anywhere, intra-crate or external, in production code — an honest
empty production tree. Exercised only by this crate's own tests.

## Callee Tree

- **External:** `<[u8]>::get` (range-checked slice access), `<[u8]>::try_into`
  (array conversion), `i64::from_le_bytes` (all standard-library, `src/lib.rs:164,169`)
