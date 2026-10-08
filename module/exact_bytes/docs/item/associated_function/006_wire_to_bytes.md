# 006: Wire::to_bytes

## Representation

Encode to a fixed-size byte array: an 8-byte little-endian `minor`, then the
`scale` byte, then the `kind` byte.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:168-175`

```rust
pub fn to_bytes( self ) -> [ u8; Self::ENCODED_LEN ]
{
  let mut out = [ 0_u8; Self::ENCODED_LEN ];
  out[ 0 .. 8 ].copy_from_slice( &self.minor.to_le_bytes() );
  out[ 8 ] = self.scale;
  out[ 9 ] = self.kind;
  out
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 168-175 | Declaration |
| `tests/wire_roundtrip_test.rs` | 55,66,76,147 | Encoding a `Wire` before tampering a byte, before truncating, for a raw-byte round-trip, and before extending the slice |

No production function in `exact_bytes` or `exact_arith` calls `to_bytes` —
only test code does.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `tests/wire_roundtrip_test.rs` | Exercised by 4 of the crate's 14 tests |

## Caller Tree

No caller anywhere, intra-crate or external, in production code — an honest
empty production tree. Exercised only by this crate's own tests.

## Callee Tree

- **External:** `i64::to_le_bytes` (`self.minor.to_le_bytes()`) and
  `<[u8]>::copy_from_slice` (both standard-library methods, `src/lib.rs:171`)
