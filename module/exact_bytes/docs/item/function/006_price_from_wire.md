# 006: price_from_wire

## Representation

Decode a price value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:228-239`

```rust
pub fn price_from_wire( w : Wire ) -> Result< Price, WireError >
{
  if w.kind != KIND_PRICE
  {
    return Err( WireError::BadKind );
  }
  if w.scale != exact_scale::MONEY_SCALE as u8
  {
    return Err( WireError::BadScale );
  }
  Price::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 228-239 | Declaration |
| `tests/wire_roundtrip_test.rs:36` | — | Decoding in the price round-trip test |
| `exact_arith/src/lib.rs:117` | — | Facade re-export |

`exact_arith`'s own facade test does not call this function.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own price round-trip test |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- `kind_error_to_wire_error` (`src/lib.rs:75`, private — no Item Instance of its own)
- **External:** `exact_kind::Price::from_minor` (`src/lib.rs:238`)
