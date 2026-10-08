# 006: price_from_wire

## Representation

Decode a price value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:247-251`

```rust
pub fn price_from_wire( w : Wire ) -> Result< Price, WireError >
{
  check_header( w, KIND_PRICE )?;
  Price::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 247-251 | Declaration |
| `tests/wire_roundtrip_test.rs:36` | — | Decoding in the price round-trip test |
| `exact_arith/src/lib.rs:129` | — | Facade re-export |

`exact_arith`'s own facade test does not call this function.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by its own price round-trip test |
| `exact_arith` | `src/lib.rs` | Re-export only — not referenced by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- `check_header` (`src/lib.rs:104`, private — no Item Instance of its own): the kind check (`BadKind`), then the scale check against `SCALE_BYTE` (`BadScale`)
- `kind_error_to_wire_error` (`src/lib.rs:92`, private — no Item Instance of its own)
- **External:** `exact_kind::Price::from_minor` (`src/lib.rs:250`)
