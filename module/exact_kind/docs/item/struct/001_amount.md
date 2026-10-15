# 001: Amount

## Representation

A fixed-point amount of kind `K` at scale `SCALE`: one count of minor units,
so the denoted value is `minor × 10⁻ˢᶜᵃˡᵉ`, and a zero-sized kind marker.
The one struct behind all three kinds — [Decimal](../type_alias/004_decimal.md),
[Qty](../type_alias/005_qty.md) and [PriceAt](../type_alias/006_price_at.md)
are its aliases for [MoneyKind](002_money_kind.md), [QtyKind](003_qty_kind.md)
and [PriceKind](004_price_kind.md). `PhantomData< K >` takes no space, so
every amount is one `i64`; but `Amount< MoneyKind, 2 >` and
`Amount< QtyKind, 2 >` are two types, so two kinds never mix, as two scales
never do (→ [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

Both fields are private. Each kind's members live in its own `impl` block on
its alias, so the kinds keep different APIs: `Qty` has no `MIN` or
`checked_neg`, and `PriceAt` has no `from_int` or `whole`.

Two private members, in an `impl` block over every `K` (`src/lib.rs:210-226`),
carry the old `value` field's role: `value()` reads an amount as money —
`Qty::as_decimal` and every `Qty`/`PriceAt` delegation use it — and
`from_value( Decimal )` builds a kind from money unchecked. `from_value` is
private, so outside this crate a `Qty` still exists only after
`Qty::from_decimal` has refused a negative value
(→ [ADR-001](../../decisions/001_non_negativity_enforced_at_construction.md)).
Being private, neither has a page of its own.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:204`

```rust
#[ derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Amount< K, const SCALE : u32 >
{
  minor : Minor,
  kind : PhantomData< K >,
}
```

`Debug` is not derived: a derived one would print
`Amount { minor: …, kind: PhantomData<…> }`, so it is
[written by hand](../implementation/007_debug_for_amount.md) to print the
kind's own name. The other derives need `K` to derive them too, which each
kind marker does.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 197-208 | Declaration |
| `src/lib.rs` | 142,152,154 | The private `Sealed` hooks take and return `Amount< Self, SCALE >` |
| `src/lib.rs` | 159 | `Kind`'s doc: the kind is a type parameter of `Amount` |
| `src/lib.rs` | 210-226 | The private `value()` and `from_value()`, for every `K` |
| `src/lib.rs` | 230-237 | `Debug`, for every `K : Kind` |
| `src/lib.rs` | 260,653,892 | `Decimal`, `Qty` and `PriceAt`, its aliases |
| `src/lib.rs` | 1037-1078 | `+` and `-`, four generic impls on `Amount< K, SCALE >` and on `Result< Amount< K, SCALE >, KindError >` |
| `tests/amount_test.rs` | 25-27 | Each alias is `Amount` of its own kind |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

No crate outside `exact_kind` names `Amount`: every caller writes an alias.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The one struct behind `Decimal`, `Qty` and `PriceAt`; exercised by `tests/amount_test.rs` |
| `exact_arith` | `src/lib.rs` | Re-export |
