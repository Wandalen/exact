# 029: Sub::sub for Result

## Representation

The one method of [impl Sub< Amount > for Result](../implementation/011_sub_for_result.md): `( a + b ) - c`.
Passes an error on the left through with `?`, then calls `K::checked_sub`, the
kind's private hook, which calls that kind's own inherent `checked_sub`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:1074`

```rust
fn sub( self, rhs : Amount< K, SCALE > ) -> Self::Output
{
  K::checked_sub( self?, rhs )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1074-1077 | Declaration |
| `tests/operator_test.rs` | 29,32 | `a + b - c`; `Money::MAX + Money::EPSILON - Money::EPSILON` keeps the first error |
| `tests/operator_test.rs` | 57,58,64,65 | `Qty` and `PriceAt` chains ending in `-` |
| `smoke_exact_market_split/src/lib.rs:186` | — | Scene 002's `( a + b - total )`, at scale 2 |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by `tests/operator_test.rs` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: scene 002, through `exact_arith` |

## Caller Tree

- Every `( a + b ) - c` on an amount — the uses above; never named directly

## Callee Tree

- `MoneyKind`'s hook (`src/lib.rs:989`) → [Decimal::checked_sub](006_checked_sub_decimal.md)
- `QtyKind`'s hook (`src/lib.rs:996`) → [Qty::checked_sub](017_checked_sub_qty.md)
- `PriceKind`'s hook (`src/lib.rs:1003`) → `PriceAt::checked_sub` ([PriceAt](../type_alias/006_price_at.md))
