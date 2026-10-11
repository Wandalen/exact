# 004: Wire::scale

## Representation

The scale this value was written at.

**Never called in production.** `check_header`, which all three
`*_from_wire` functions call, reads `w.scale` as a direct private-field access
(`src/lib.rs:113`) rather than through this accessor, and
`tests/wire_roundtrip_test.rs`'s one scale-mismatch test
(`a_mismatched_scale_byte_is_refused`, line 52) tampers the raw byte array
directly (`bytes[ 8 ] = 9`). Its one caller is `exact_arith`'s any-scale test,
which reads the scale byte back to show a value at scale 2 is written as `2`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:157-160`

```rust
pub const fn scale( self ) -> u8
{
  self.scale
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 157-160 | Declaration |
| `exact_arith/tests/any_scale_test.rs:70` | — | The scale byte of money encoded at scale 2 is `2` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Declared but unused even by this crate's own tests |
| `exact_arith` | `tests/any_scale_test.rs` | Reads the scale byte back, test only |

## Caller Tree

No caller in production code — an honest empty tree. The one call is a test
in `exact_arith`, out of scope for this tree.

## Callee Tree

No callees — returns the private `scale` field directly.
