# 011: Qty::from_minor

## Representation

Build from a count of minor units, refusing a negative one or one past the
declared ceiling.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:456`

```rust
pub const fn from_minor( minor : Backing ) -> Result< Self, KindError >
{
  match Decimal::from_minor( minor )
  {
    Ok( value ) => Self::from_decimal( value ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 456 | Declaration |
| `tests/non_negative_test.rs` | throughout | Negative/ceiling-breach checks |
| `exact_bytes/src/lib.rs:233` | — | `qty_from_wire` |
| `exact_dust/src/lib.rs:245,255` | — | `qty_dust_split`/`qty_dust_split_into`, passed as `make` to the private `split_with`/`split_into_with` |
| `exact_ratio/src/lib.rs:195,238` | — | `qty_mul_ratio`/`qty_div_round` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own tests |
| `exact_bytes`, `exact_dust`, `exact_ratio` | `src/lib.rs` | **Production** — re-wrap a computed minor-unit count back into a range- and sign-checked `Quantity` |

## Caller Tree

- **External:** `exact_bytes::qty_from_wire` (`exact_bytes/src/lib.rs:233`)
- **External:** `exact_dust::qty_dust_split` (`exact_dust/src/lib.rs:245`), `qty_dust_split_into` (`:255`), each passing it as `make`
- **External:** `exact_ratio::qty_mul_ratio` (`exact_ratio/src/lib.rs:195`), `qty_div_round` (`:238`)

No intra-crate caller.

## Callee Tree

- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:458`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:460`)
