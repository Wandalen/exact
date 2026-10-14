# 009: impl Add< Amount > for Result

## Representation

`( a + b ) + c`: `+` with a `Result` on the left, so a chain carries on
without a `?` per step. An error on the left is passed on untouched (`self?`),
so a chain stops at its first error; otherwise `c` is added with the kind's
own `checked_add`. Only left to right: `a + ( b + c )` has no impl. Allowed by
the orphan rule because `Amount` is this crate's own type and appears in the
impl's trait parameter.

| Kind | Hook | Calls | Error a caller can see |
|---|---|---|---|
| `Decimal` | `src/lib.rs:988` | [Decimal::checked_add](../associated_function/005_checked_add_decimal.md) | the error on the left, or `ExceedsCeiling`, `Overflow` |
| `Qty` | `src/lib.rs:995` | [Qty::checked_add](../associated_function/016_checked_add_qty.md) | the error on the left, or `ExceedsCeiling`, `Overflow` |
| `PriceAt` | `src/lib.rs:1002` | `PriceAt::checked_add` ([PriceAt](../type_alias/006_price_at.md)) | the error on the left, or `ExceedsCeiling`, `Overflow` |

The hooks are the private `Sealed` impls, so they have no pages of their own;
each is one line calling the inherent method (→ [ADR-005](../../decisions/005_checked_operators_return_a_result.md), [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:1048`

```rust
impl< K : Kind, const SCALE : u32 > core::ops::Add< Amount< K, SCALE > > for Result< Amount< K, SCALE >, KindError >
{
  type Output = Result< Amount< K, SCALE >, KindError >;

  fn add( self, rhs : Amount< K, SCALE > ) -> Self::Output
  {
    K::checked_add( self?, rhs )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1047-1056 | Declaration |
| `src/lib.rs` | 1018 | Doc example: the second `+` of `( a + b + c )?` |
| `tests/operator_test.rs` | 28,30 | `a + b + c`; `Money::MAX + Money::EPSILON + Money::EPSILON` keeps the first error |
| `tests/operator_test.rs` | 56,59,63 | `Qty` and `PriceAt` chains; a `Negative` from `q( "1" ) - q( "2" )` carried through `+ q( "5" )` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by `tests/operator_test.rs` |

No caller outside this crate's own doc example and tests. The impl travels
with the types, so `exact_arith`'s re-export carries it without a line of
its own.
