# 026: Add::add for Amount

## Representation

The one method of [impl Add for Amount](../implementation/008_add_for_amount.md): `a + b`.
Calls `K::checked_add`, the
kind's private hook, which calls that kind's own inherent `checked_add`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:1041`

```rust
fn add( self, rhs : Self ) -> Self::Output
{
  K::checked_add( self, rhs )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1041-1044 | Declaration |
| `src/lib.rs` | 1006-1036 | Doc comment: `( a + b + c )?` on `Decimal< 2 >` (1018) as a runnable example; `Decimal< 2 >` plus `Decimal< 8 >` (1030) and a `Price` plus a `Quantity` (1035) as `compile_fail` examples |
| `tests/operator_test.rs` | 17,19 | `a + b` equals `a.checked_add( b )` |
| `tests/operator_test.rs` | 28-30,32 | The first `+` of each chain; `Money::MAX + Money::EPSILON` is the first error |
| `tests/operator_test.rs` | 44,55,56,58,60,63,65 | `PriceAt`'s and `Qty`'s `+`, including `Quantity::MAX + Quantity::EPSILON`, past the ceiling |
| `smoke_exact_market_split/src/lib.rs:71` | — | Ten tenths summed with `+` on `Money` |
| `smoke_exact_market_split/src/lib.rs:186` | — | Scene 002's `a + b`, at scale 2 |
| `smoke_exact_market_split/src/lib.rs:333,414` | — | Split shares recombined with `try_fold( …, \| a, b \| a + b )`, on `Money` and on `Qty< 8 >` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by `tests/operator_test.rs` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: sums and recombined splits, through `exact_arith` |

## Caller Tree

- Every `a + b` on an amount — the uses above; never named directly

## Callee Tree

- `MoneyKind`'s hook (`src/lib.rs:988`) → [Decimal::checked_add](005_checked_add_decimal.md)
- `QtyKind`'s hook (`src/lib.rs:995`) → [Qty::checked_add](016_checked_add_qty.md)
- `PriceKind`'s hook (`src/lib.rs:1002`) → `PriceAt::checked_add` ([PriceAt](../type_alias/006_price_at.md))
