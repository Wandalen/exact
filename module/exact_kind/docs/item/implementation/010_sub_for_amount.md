# 010: impl Sub for Amount

## Representation

`a - b` for two amounts of one kind at one scale: that kind's own
`checked_sub`, spelled as an operator. For `Decimal` and `PriceAt` a
difference below zero is a value; for `Qty` it is `Err( Negative )`, because
`QtyKind`'s hook calls `Qty::checked_sub`, which refuses it. Where a checked
operator matters most: the generic impl does not flatten the kinds' rules into
one.

| Kind | Hook | Calls | Error a caller can see |
|---|---|---|---|
| `Decimal` | `src/lib.rs:989` | [Decimal::checked_sub](../associated_function/006_checked_sub_decimal.md) | `ExceedsCeiling`, `Overflow` — a difference below zero is a value |
| `Qty` | `src/lib.rs:996` | [Qty::checked_sub](../associated_function/017_checked_sub_qty.md) | `Negative` below zero, `ExceedsCeiling`, `Overflow` |
| `PriceAt` | `src/lib.rs:1003` | `PriceAt::checked_sub` ([PriceAt](../type_alias/006_price_at.md)) | `ExceedsCeiling`, `Overflow` |

The hooks are the private `Sealed` impls, so they have no pages of their own;
each is one line calling the inherent method (→ [ADR-005](../../decisions/005_checked_operators_return_a_result.md), [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:1059`

```rust
impl< K : Kind, const SCALE : u32 > core::ops::Sub for Amount< K, SCALE >
{
  type Output = Result< Self, KindError >;

  fn sub( self, rhs : Self ) -> Self::Output
  {
    K::checked_sub( self, rhs )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1058-1067 | Declaration |
| `src/lib.rs` | 1021 | Doc example: `one - two` on `Qty< 8 >` is `Negative` |
| `tests/operator_test.rs` | 18,20 | `a - b` equals `a.checked_sub( b )` |
| `tests/operator_test.rs` | 41-42 | `Qty`'s `-`: `Negative` below zero, a value otherwise |
| `tests/operator_test.rs` | 57,59,62,64 | The first `-` of `Qty` and `PriceAt` chains; `p( "5.00" ) - p( "1.25" )` equals `checked_sub` |
| `smoke_exact_market_split/src/lib.rs:304` | — | Step 3: `held - taken` on `Quantity` is refused as `Negative` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by `tests/operator_test.rs` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: a refused withdrawal, through `exact_arith` |

The demo lane is the one caller outside this crate; production code still
goes through `exact_add`'s `money_*`/`qty_*`/`price_*` functions or the
`checked_*` methods. The impl travels with the types, so `exact_arith`'s
re-export carries it without a line of its own.
