# 011: impl Sub< Amount > for Result

## Representation

`( a + b ) - c`: `-` with a `Result` on the left. An error on the left is
passed on untouched (`self?`); otherwise `c` is subtracted with the kind's own
`checked_sub`, so a `Qty` chain that goes below zero still ends in `Negative`.

| Kind | Hook | Calls | Error a caller can see |
|---|---|---|---|
| `Decimal` | `src/lib.rs:989` | [Decimal::checked_sub](../associated_function/006_checked_sub_decimal.md) | the error on the left, or `ExceedsCeiling`, `Overflow` |
| `Qty` | `src/lib.rs:996` | [Qty::checked_sub](../associated_function/017_checked_sub_qty.md) | the error on the left, or `Negative`, `ExceedsCeiling`, `Overflow` |
| `PriceAt` | `src/lib.rs:1003` | `PriceAt::checked_sub` ([PriceAt](../type_alias/006_price_at.md)) | the error on the left, or `ExceedsCeiling`, `Overflow` |

The hooks are the private `Sealed` impls, so they have no pages of their own;
each is one line calling the inherent method (→ [ADR-005](../../decisions/005_checked_operators_return_a_result.md), [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:1070`

```rust
impl< K : Kind, const SCALE : u32 > core::ops::Sub< Amount< K, SCALE > > for Result< Amount< K, SCALE >, KindError >
{
  type Output = Result< Amount< K, SCALE >, KindError >;

  fn sub( self, rhs : Amount< K, SCALE > ) -> Self::Output
  {
    K::checked_sub( self?, rhs )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1069-1078 | Declaration |
| `tests/operator_test.rs` | 29,32 | `a + b - c`; `Money::MAX + Money::EPSILON - Money::EPSILON` keeps the first error |
| `tests/operator_test.rs` | 57,58,64,65 | `Qty` and `PriceAt` chains ending in `-` |
| `smoke_exact_market_split/src/lib.rs:186` | — | Scene 002's `( a + b - total )`, at scale 2 |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by `tests/operator_test.rs` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: scene 002, through `exact_arith` |

The demo lane is the one caller outside this crate; production code still
goes through `exact_add`'s `money_*`/`qty_*`/`price_*` functions or the
`checked_*` methods. The impl travels with the types, so `exact_arith`'s
re-export carries it without a line of its own.
