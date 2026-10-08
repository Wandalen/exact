# 004: qty_from_wire

## Representation

Decode a quantity value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:230-234`

```rust
pub fn qty_from_wire( w : Wire ) -> Result< Quantity, WireError >
{
  check_header( w, KIND_QTY )?;
  Quantity::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 230-234 | Declaration |
| `tests/wire_roundtrip_test.rs:26,44,96` | — | Decoding in the quantity round-trip, cross-kind-rejection, and negative-value tests |
| `exact_arith/src/lib.rs:131` | — | Facade re-export |

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

- `check_header` (`src/lib.rs:105`, private — no Item Instance of its own): the kind check (`BadKind`), then the scale check against `SCALE_BYTE` (`BadScale`)
- `kind_error_to_wire_error` (`src/lib.rs:92`, private — no Item Instance of its own)
- **External:** `exact_kind::Quantity::from_minor` (`src/lib.rs:233`)
