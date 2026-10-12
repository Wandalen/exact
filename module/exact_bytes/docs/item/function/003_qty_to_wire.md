# 003: qty_to_wire

## Representation

Encode a quantity to its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:219-222`

```rust
pub fn qty_to_wire< const S : u32 >( v : Qty< S > ) -> Wire
{
  Wire { minor : v.minor(), scale : scale_byte::< S >(), kind : KIND_QTY }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 219-222 | Declaration |
| `tests/wire_roundtrip_test.rs:25,47` | — | Encoding in the quantity round-trip test and the cross-kind-rejection test |
| `exact_arith/src/lib.rs:133` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:404` | — | Step 7 encodes a satoshi quantity, scale byte `8` |

Unlike [`money_to_wire`](001_money_to_wire.md), `exact_arith`'s own facade
test does not call this function — only the Money roundtrip is exercised at
the facade level.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: step 7's wire check |

## Caller Tree

- **External:** `smoke_exact_market_split::per_asset_trade` (`smoke_exact_market_split/src/lib.rs:404`) — demo-lane code, not production

No production caller, and no test-only exception in the facade's own test
suite (contrast [money_to_wire](001_money_to_wire.md)).

## Callee Tree

- `scale_byte::< S >()` (`src/lib.rs:50`, private — no Item Instance of its own): the record's scale byte, `S` itself; its inline `const` assertion refuses at compile time a scale the `u8` cast would cut
- **External:** `exact_kind::Qty::minor` (`v.minor()`, `src/lib.rs:221`)
