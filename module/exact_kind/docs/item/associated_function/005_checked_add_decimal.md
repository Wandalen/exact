# 005: Decimal::checked_add

## Representation

Add two values of the same scale, refusing a result past the declared
ceiling.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:246`

```rust
pub const fn checked_add( self, rhs : Self ) -> Result< Self, KindError >
{
  match minor_checked_add( self.minor, rhs.minor )
  {
    Ok( sum ) => Self::from_minor( minor_to_i64( sum ) ),
    Err( e ) => Err( kind_overflow( e ) ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 246,505 | Declaration; `Qty::checked_add`'s delegation |
| `tests/checked_arithmetic_test.rs` | throughout | Ceiling-breach and ordinary-sum checks |
| `exact_add/src/lib.rs:55,130` | — | `money_add`, `money_saturating_add` (`price_add` reaches it through `Price::checked_add`) |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::checked_add`; exercised by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `money_add` (and, through `Price::checked_add`, `price_add`), and the checked attempt `money_saturating_add` clamps on failure |

## Caller Tree

- [Qty::checked_add](016_checked_add_qty.md) (`src/lib.rs:505`)
- [Price::checked_add](../struct/003_price.md) (`src/lib.rs:673`), which `exact_add::price_add` calls
- **External:** `exact_add::money_add` (`exact_add/src/lib.rs:55`), `money_saturating_add` (`:130`)

## Callee Tree

- **External:** `exact_minor::minor_checked_add` — `minor_checked_add( self.minor, rhs.minor )`, then `minor_to_i64`; a failure goes through the private `kind_overflow`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:250`)
