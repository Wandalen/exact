# 003: qty_to_wire

## Representation

Encode a quantity to its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:190-193`

```rust
pub fn qty_to_wire( v : Quantity ) -> Wire
{
  Wire { minor : v.minor(), scale : exact_scale::MONEY_SCALE as u8, kind : KIND_QTY }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 190-193 | Declaration |
| `tests/wire_roundtrip_test.rs:24,46` | — | Encoding in the quantity round-trip test and the cross-kind-rejection test |
| `exact_arith/src/lib.rs:114` | — | Facade re-export |

Unlike [`money_to_wire`](001_money_to_wire.md), `exact_arith`'s own facade
test does not call this function — only the Money roundtrip is exercised at
the facade level.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, with no
test-only exception even in the facade's own test suite (contrast
[money_to_wire](001_money_to_wire.md)).

## Callee Tree

- **External:** `exact_kind::Quantity::minor` (`v.minor()`, `src/lib.rs:192`)
