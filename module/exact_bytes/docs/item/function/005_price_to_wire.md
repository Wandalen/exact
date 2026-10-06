# 005: price_to_wire

## Representation

Encode a price to its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:231-234`

```rust
pub fn price_to_wire( v : Price ) -> Wire
{
  Wire { minor : v.minor(), scale : exact_scale::MONEY_SCALE as u8, kind : KIND_PRICE }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 231-234 | Declaration |
| `tests/wire_roundtrip_test.rs:34` | — | Encoding in the price round-trip test |
| `exact_arith/src/lib.rs:130` | — | Facade re-export |

`exact_arith`'s own facade test does not call this function — only the Money
roundtrip is exercised at the facade level.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own price round-trip test |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `exact_kind::Price::minor` (`v.minor()`, `src/lib.rs:233`)
