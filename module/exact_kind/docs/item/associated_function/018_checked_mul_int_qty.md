# 018: Qty::checked_mul_int

## Representation

Multiply by a dimensionless non-negative integer. Unlike
[`Decimal::checked_mul_int`](007_checked_mul_int_decimal.md), a negative `n`
here produces `KindError::Negative` rather than a negative result.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:540`

```rust
pub const fn checked_mul_int( self, n : Backing ) -> Result< Self, KindError >
{
  match self.value.checked_mul_int( n )
  {
    Ok( value ) => Self::from_decimal( value ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 540 | Declaration |
| `tests/non_negative_test.rs` | throughout | Ordinary, zero, and negative-`n` checks |

No file outside `exact_kind` calls `Qty::checked_mul_int` — an honest gap,
matching [Decimal::checked_mul_int](007_checked_mul_int_decimal.md).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own tests, including the negative-`n`-refusal case |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- [Decimal::checked_mul_int](007_checked_mul_int_decimal.md) (`src/lib.rs:542`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:544`)
