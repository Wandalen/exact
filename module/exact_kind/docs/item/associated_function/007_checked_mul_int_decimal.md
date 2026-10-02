# 007: Decimal::checked_mul_int

## Representation

Multiply by a dimensionless integer, holding the scale.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:263`

```rust
pub const fn checked_mul_int( self, n : Backing ) -> Result< Self, KindError >
{
  let Some( minor ) = self.minor.checked_mul( n )
  else
  {
    return Err( KindError::Overflow { operation : "mul_int" } );
  };
  Self::from_minor( minor )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 263,519 | Declaration; `Qty::checked_mul_int`'s delegation |
| `tests/checked_arithmetic_test.rs` | throughout | Overflow and ceiling-breach checks |

No file outside `exact_kind` calls `Decimal::checked_mul_int` directly — an
honest gap. No downstream crate currently scales a `Money`/`Price` by a bare
integer (only `exact_ratio`'s rational scaling exists today).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::checked_mul_int`; exercised by its own tests |

## Caller Tree

- [Qty::checked_mul_int](018_checked_mul_int_qty.md) (`src/lib.rs:519`)

No external caller anywhere in the workspace.

## Callee Tree

- **External:** `i64::checked_mul` — `self.minor.checked_mul( n )`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:270`)
