# 027: Add::add for Result

## Representation

The one method of [impl Add< Amount > for Result](../implementation/009_add_for_result.md): `( a + b ) + c`.
Passes an error on the left through with `?`, then calls `K::checked_add`, the
kind's private hook, which calls that kind's own inherent `checked_add`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:1052`

```rust
fn add( self, rhs : Amount< K, SCALE > ) -> Self::Output
{
  K::checked_add( self?, rhs )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1052-1055 | Declaration |
| `src/lib.rs` | 1018 | Doc example: the second `+` of `( a + b + c )?` |
| `tests/operator_test.rs` | 28,30 | `a + b + c`; `Money::MAX + Money::EPSILON + Money::EPSILON` keeps the first error |
| `tests/operator_test.rs` | 56,59,63 | `Qty` and `PriceAt` chains; a `Negative` from `q( "1" ) - q( "2" )` carried through `+ q( "5" )` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by `tests/operator_test.rs` |

## Caller Tree

- Every `( a + b ) + c` on an amount — the uses above; never named directly

## Callee Tree

- `MoneyKind`'s hook (`src/lib.rs:988`) → [Decimal::checked_add](005_checked_add_decimal.md)
- `QtyKind`'s hook (`src/lib.rs:995`) → [Qty::checked_add](016_checked_add_qty.md)
- `PriceKind`'s hook (`src/lib.rs:1002`) → `PriceAt::checked_add` ([PriceAt](../type_alias/006_price_at.md))
