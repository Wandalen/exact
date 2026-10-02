# 002: money_from_wire

## Representation

Decode a money value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:175-186`

```rust
pub fn money_from_wire( w : Wire ) -> Result< Money, WireError >
{
  if w.kind != KIND_MONEY
  {
    return Err( WireError::BadKind );
  }
  if w.scale != exact_scale::MONEY_SCALE as u8
  {
    return Err( WireError::BadScale );
  }
  Money::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 175-186 | Declaration |
| `tests/wire_roundtrip_test.rs` | 16,47,58,78,88,99 | Decoding across 6 of the crate's 8 tests, covering every failure mode and the success path |
| `exact_arith/src/lib.rs:109` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:42` | — | Decoding in the facade's own end-to-end settlement test |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | The crate's most-tested function — exercises every `WireError` variant except `Truncated` |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-exported, and actually called in the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external, in **production** code — an
honest empty tree. `exact_arith/tests/facade_test.rs:42` calls it, but that
is a test file in a different crate, out of scope for this tree.

## Callee Tree

- `kind_error_to_wire_error` (`src/lib.rs:75`, private — no Item Instance of its own)
- **External:** `exact_kind::Money::from_minor` (`src/lib.rs:185`)
