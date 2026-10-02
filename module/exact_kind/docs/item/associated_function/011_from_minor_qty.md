# 011: Qty::from_minor

## Representation

Build from a count of minor units, refusing a negative one or one past the
declared ceiling.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:433`

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
| `src/lib.rs` | 433 | Declaration |
| `tests/non_negative_test.rs` | throughout | Negative/ceiling-breach checks |
| `exact_bytes/src/lib.rs:213` | — | `qty_from_wire` |
| `exact_dust/src/lib.rs:201,216` | — | `qty_dust_split`/`qty_dust_split_into` |
| `exact_ratio/src/lib.rs:156,202` | — | `qty_mul_ratio`/`qty_div_round` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own tests |
| `exact_bytes`, `exact_dust`, `exact_ratio` | `src/lib.rs` | **Production** — re-wrap a computed minor-unit count back into a range- and sign-checked `Quantity` |

## Caller Tree

- **External:** `exact_bytes::qty_from_wire` (`exact_bytes/src/lib.rs:213`)
- **External:** `exact_dust::qty_dust_split` (`exact_dust/src/lib.rs:201`), `qty_dust_split_into` (`:216`)
- **External:** `exact_ratio::qty_mul_ratio` (`exact_ratio/src/lib.rs:156`), `qty_div_round` (`:202`)

No intra-crate caller.

## Callee Tree

- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:435`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:437`)
