# 028: Sub::sub for Amount

## Representation

The one method of [impl Sub for Amount](../implementation/010_sub_for_amount.md): `a - b`.
Calls `K::checked_sub`, the
kind's private hook, which calls that kind's own inherent `checked_sub`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:1063`

```rust
fn sub( self, rhs : Self ) -> Self::Output
{
  K::checked_sub( self, rhs )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1063-1066 | Declaration |
| `src/lib.rs` | 1021 | Doc example: `one - two` on `Qty< 8 >` is `Negative` |
| `tests/operator_test.rs` | 18,20 | `a - b` equals `a.checked_sub( b )` |
| `tests/operator_test.rs` | 41-42 | `Qty`'s `-`: `Negative` below zero, a value otherwise |
| `tests/operator_test.rs` | 57,59,62,64 | The first `-` of `Qty` and `PriceAt` chains; `p( "5.00" ) - p( "1.25" )` equals `checked_sub` |
| `smoke_exact_market_split/src/lib.rs:304` | — | Step 3: `held - taken` on `Quantity` is refused as `Negative` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by `tests/operator_test.rs` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: a refused withdrawal, through `exact_arith` |

## Caller Tree

- Every `a - b` on an amount — the uses above; never named directly

## Callee Tree

- `MoneyKind`'s hook (`src/lib.rs:989`) → [Decimal::checked_sub](006_checked_sub_decimal.md)
- `QtyKind`'s hook (`src/lib.rs:996`) → [Qty::checked_sub](017_checked_sub_qty.md)
- `PriceKind`'s hook (`src/lib.rs:1003`) → `PriceAt::checked_sub` ([PriceAt](../type_alias/006_price_at.md))
