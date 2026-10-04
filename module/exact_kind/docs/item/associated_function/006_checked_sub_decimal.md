# 006: Decimal::checked_sub

## Representation

Subtract two values of the same scale, refusing a result past the declared
ceiling.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:260`

```rust
pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
{
  match minor_checked_sub( self.minor, rhs.minor )
  {
    Ok( diff ) => Self::from_minor( minor_to_i64( diff ) ),
    Err( e ) => Err( kind_overflow( e ) ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 260,519 | Declaration; `Qty::checked_sub`'s delegation |
| `tests/checked_arithmetic_test.rs` | throughout | Floor-breach and ordinary-difference checks |
| `exact_add/src/lib.rs:65` | — | `money_sub` (`price_sub` reaches it through `Price::checked_sub`) |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::checked_sub`; exercised by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `money_sub` (and, through `Price::checked_sub`, `price_sub`) |

## Caller Tree

- [Qty::checked_sub](017_checked_sub_qty.md) (`src/lib.rs:519`)
- [Price::checked_sub](../struct/003_price.md) (`src/lib.rs:687`), which `exact_add::price_sub` calls
- **External:** `exact_add::money_sub` (`exact_add/src/lib.rs:65`)

## Callee Tree

- **External:** `exact_minor::minor_checked_sub` — `minor_checked_sub( self.minor, rhs.minor )`, then `minor_to_i64`; a failure goes through the private `kind_overflow`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:264`)
