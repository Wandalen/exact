# 016: Qty::checked_add

## Representation

Add two quantities. Cannot produce `KindError::Negative` — two non-negative
values do not sum below zero — so only the range errors of the underlying
decimal add can surface.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:753`

```rust
pub const fn checked_add( self, rhs : Self ) -> Result< Self, KindError >
{
  match self.value().checked_add( rhs.value() )
  {
    Ok( value ) => Self::from_decimal( value ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 753,995 | Declaration; `QtyKind`'s hook, which `+` on `Qty` and on a `Result< Qty, KindError >` calls |
| `tests/non_negative_test.rs` | throughout | Ordinary-sum and ceiling-breach checks |
| `exact_add/src/lib.rs:75` | — | `qty_add`'s entire body |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs the `+` operator on `Qty`; exercised by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `qty_add` |

## Caller Tree

- **External:** `exact_add::qty_add` (`exact_add/src/lib.rs:75`)

- **Intra-crate:** `QtyKind`'s `checked_add` hook (`src/lib.rs:995`), through which [`+` on `Amount`](026_add_for_amount.md) and [`+` on a `Result`](027_add_for_result.md) reach it for `Qty`

## Callee Tree

- [Decimal::checked_add](005_checked_add_decimal.md) (`src/lib.rs:755`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:757`)
