# 004: Wire::scale

## Representation

The scale this value was written at.

**Never actually called — the same verified discrepancy as `Wire::minor`.**
All three `*_from_wire` functions read `w.scale` as a direct private-field
access (`src/lib.rs:194,222,247`) rather than through this accessor. No test
calls it either — `tests/wire_roundtrip_test.rs`'s one scale-mismatch test
(`a_mismatched_scale_byte_is_refused`, line 52) tampers the raw byte array
directly (`bytes[ 8 ] = 9`) rather than reading a `Wire`'s `.scale()` back.
Confirmed via an exhaustive grep for `.scale(` across `exact_bytes` and
`exact_arith`: zero matches anywhere.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:134-137`

```rust
pub const fn scale( self ) -> u8
{
  self.scale
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 134-137 | Declaration |

No file anywhere calls this method — an honest empty finding, identical in
shape to [Wire::minor](003_wire_minor.md): declared, exported, never called.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Declared but unused even by this crate's own tests |

## Caller Tree

No caller anywhere, intra-crate or external, test or production — an honest
empty tree.

## Callee Tree

No callees — returns the private `scale` field directly.
