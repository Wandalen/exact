# 006: price_from_wire

## Representation

Decode a price value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:250-254`

```rust
pub fn price_from_wire< const S : u32 >( w : Wire ) -> Result< PriceAt< S >, WireError >
{
  check_header( w, KIND_PRICE, scale_byte::< S >() )?;
  PriceAt::< S >::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 250-254 | Declaration |
| `tests/wire_roundtrip_test.rs:37` | — | Decoding in the price round-trip test |
| `tests/wire_roundtrip_test.rs:161,162,166` | — | A price at scale 2 decoded, and refused at scale 6 with `BadScale`; one at scale 8 decoded from raw bytes |
| `exact_arith/src/lib.rs:130` | — | Facade re-export |

`exact_arith`'s own facade test does not call this function.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own price round-trip test |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- `check_header` (`src/lib.rs:107`, private — no Item Instance of its own): the kind check (`BadKind`), then the scale check against the expected scale (`BadScale`)
- `scale_byte::< S >()` (`src/lib.rs:50`, private — no Item Instance of its own): the expected scale byte, `S` itself; its inline `const` assertion refuses at compile time a scale the `u8` cast would cut
- `kind_error_to_wire_error` (`src/lib.rs:94`, private — no Item Instance of its own)
- **External:** `exact_kind::PriceAt::from_minor` (`src/lib.rs:253`)
