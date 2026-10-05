# 001: Decimal::from_minor

## Representation

Build from a count of minor units, refusing anything past the declared
ceiling. The one range gate every other constructor and operation funnels
through.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:199`

```rust
pub const fn from_minor( minor : Backing ) -> Result< Self, KindError >
{
  if minor > CEILING_MINOR_UNITS || minor < -CEILING_MINOR_UNITS
  {
    return Err( KindError::ExceedsCeiling { minor } );
  }
  Ok( Self { minor : minor_from_i64( minor ) } )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 199,222,250,264,283,298,369,450 | Range gate inside `from_int`, `checked_add`, `checked_sub`, `checked_mul_int`, `checked_neg`, `parse`, and `Qty::from_minor` |
| `tests/checked_arithmetic_test.rs` | throughout | Direct construction at and past boundaries |
| `exact_bytes/src/lib.rs:198,251` | — | `money_from_wire`/`price_from_wire` |
| `exact_dust/src/lib.rs:167,183` | — | `money_dust_split`/`money_dust_split_into` |
| `exact_ratio/src/lib.rs:156,180,201` | — | `money_mul_ratio`, `price_mul_ratio`, `money_div_round` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The range gate behind every constructor and checked operation |
| `exact_bytes`, `exact_dust`, `exact_ratio` | `src/lib.rs` | Re-wrap a computed minor-unit count back into a range-checked `Money`/`Price` |

## Caller Tree

- [Decimal::from_int](002_from_int_decimal.md) (`src/lib.rs:222`)
- [Decimal::checked_add](005_checked_add_decimal.md) (`src/lib.rs:250`)
- [Decimal::checked_sub](006_checked_sub_decimal.md) (`src/lib.rs:264`)
- [Decimal::checked_mul_int](007_checked_mul_int_decimal.md) (`src/lib.rs:283`)
- [Decimal::checked_neg](008_checked_neg_decimal.md) (`src/lib.rs:298`)
- [Decimal::parse](009_parse_decimal.md) (`src/lib.rs:369`)
- [Qty::from_minor](011_from_minor_qty.md) (`src/lib.rs:450`)
- **External:** `exact_bytes::money_from_wire` (`exact_bytes/src/lib.rs:198`), `exact_bytes::price_from_wire` (`:238`)
- **External:** `exact_dust::money_dust_split` (`exact_dust/src/lib.rs:167`), `money_dust_split_into` (`:172`)
- **External:** `exact_ratio::money_mul_ratio` (`exact_ratio/src/lib.rs:156`), `price_mul_ratio` (`:167`), `money_div_round` (`:189`)

This function is, by call count, the single most-relied-upon item in the
whole crate — every one of the other 5 constructors/operations on `Decimal`
funnels through it, plus 3 downstream crates call it directly to re-wrap a
computed minor count.

## Callee Tree

- None — a pure comparison against `CEILING_MINOR_UNITS` and a struct
  literal. No function call of any kind in the body.
