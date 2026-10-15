# 008: impl Add for Amount

## Representation

`a + b` for two amounts of one kind at one scale: that kind's own
`checked_add`, spelled as an operator. The `Output` is a `Result` — nothing
panics and nothing wraps, so a sum past the ceiling is `Err( ExceedsCeiling )`
the caller must handle. Both sides are `Amount< K, SCALE >` with the same `K`
and `SCALE`, so `Decimal< 2 >` plus `Decimal< 8 >`, or a `Price` plus a
`Quantity`, does not compile. One generic impl for every kind;
`K::checked_add` is the kind's private hook, which calls that kind's inherent
method, so each kind keeps its own rules.

| Kind | Hook | Calls | Error a caller can see |
|---|---|---|---|
| `Decimal` | `src/lib.rs:988` | [Decimal::checked_add](../associated_function/005_checked_add_decimal.md) | `ExceedsCeiling`, `Overflow` |
| `Qty` | `src/lib.rs:995` | [Qty::checked_add](../associated_function/016_checked_add_qty.md) | `ExceedsCeiling`, `Overflow` |
| `PriceAt` | `src/lib.rs:1002` | `PriceAt::checked_add` ([PriceAt](../type_alias/006_price_at.md)) | `ExceedsCeiling`, `Overflow` |

The hooks are the private `Sealed` impls, so they have no pages of their own;
each is one line calling the inherent method (→ [ADR-005](../../decisions/005_checked_operators_return_a_result.md), [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:1037`

```rust
impl< K : Kind, const SCALE : u32 > core::ops::Add for Amount< K, SCALE >
{
  type Output = Result< Self, KindError >;

  fn add( self, rhs : Self ) -> Self::Output
  {
    K::checked_add( self, rhs )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 1006-1045 | Declaration |
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
| `exact_kind` | `(defining crate)` | Declared here; exercised by `tests/operator_test.rs` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: sums and recombined splits, through `exact_arith` |

The demo lane is the one caller outside this crate; production code still
goes through `exact_add`'s `money_*`/`qty_*`/`price_*` functions or the
`checked_*` methods. The impl travels with the types, so `exact_arith`'s
re-export carries it without a line of its own.
