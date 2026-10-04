# 004: qty_from_wire

## Representation

Decode a quantity value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:203-214`

```rust
pub fn qty_from_wire( w : Wire ) -> Result< Quantity, WireError >
{
  if w.kind != KIND_QTY
  {
    return Err( WireError::BadKind );
  }
  if w.scale != exact_scale::MONEY_SCALE as u8
  {
    return Err( WireError::BadScale );
  }
  Quantity::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 203-214 | Declaration |
| `tests/wire_roundtrip_test.rs:26,44,96` | — | Decoding in the quantity round-trip, cross-kind-rejection, and negative-value tests |
| `exact_arith/src/lib.rs:119` | — | Facade re-export |

Unlike [`money_from_wire`](002_money_from_wire.md), `exact_arith`'s own
facade test does not call this function.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own tests, including the one test proving a negative decode is refused for a non-negative kind |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, with no
test-only exception even in the facade's own test suite.

## Callee Tree

- `kind_error_to_wire_error` (`src/lib.rs:75`, private — no Item Instance of its own)
- **External:** `exact_kind::Quantity::from_minor` (`src/lib.rs:213`)
