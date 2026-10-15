# 005: use core::marker::PhantomData

## Representation

Brings `core::marker::PhantomData` into scope, for `Amount`'s zero-sized
`kind` field: the field that makes `Amount< MoneyKind, SCALE >` and
`Amount< QtyKind, SCALE >` two types while storing nothing
(→ [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).
Every place that builds an amount from its fields writes `PhantomData` for
it. Private — not re-exported — so it has no existence outside this file.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:42`

```rust
use core::marker::PhantomData;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 207 | `Amount`'s `kind : PhantomData< K >` field |
| `src/lib.rs` | 216,224 | The private `value()` and `from_value()` build an amount |
| `src/lib.rs` | 284,287,294,297,311 | `Decimal`'s `ZERO`, `EPSILON`, `MAX`, `MIN` and `from_minor` build one |

No other file references this declaration — it is private and grants no
external visibility.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The only crate that can see this binding; not re-exported |
