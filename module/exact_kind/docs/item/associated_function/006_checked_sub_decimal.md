# 006: Decimal::checked_sub

## Representation

Subtract two values of the same scale, refusing a result past the declared
ceiling.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:246`

```rust
pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
{
  let Some( minor ) = self.minor.checked_sub( rhs.minor )
  else
  {
    return Err( KindError::Overflow { operation : "sub" } );
  };
  Self::from_minor( minor )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 246,504 | Declaration; `Qty::checked_sub`'s delegation |
| `tests/checked_arithmetic_test.rs` | throughout | Floor-breach and ordinary-difference checks |
| `exact_add/src/lib.rs:47,87` | — | `money_sub`, `price_sub` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::checked_sub`; exercised by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `money_sub`/`price_sub` |

## Caller Tree

- [Qty::checked_sub](017_checked_sub_qty.md) (`src/lib.rs:504`)
- **External:** `exact_add::money_sub` (`exact_add/src/lib.rs:47`), `price_sub` (`:87`)

## Callee Tree

- **External:** `i64::checked_sub` — `self.minor.checked_sub( rhs.minor )`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:253`)
