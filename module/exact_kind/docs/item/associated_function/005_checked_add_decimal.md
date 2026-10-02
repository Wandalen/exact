# 005: Decimal::checked_add

## Representation

Add two values of the same scale, refusing a result past the declared
ceiling.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:231`

```rust
pub const fn checked_add( self, rhs : Self ) -> Result< Self, KindError >
{
  let Some( minor ) = self.minor.checked_add( rhs.minor )
  else
  {
    return Err( KindError::Overflow { operation : "add" } );
  };
  Self::from_minor( minor )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 231,490 | Declaration; `Qty::checked_add`'s delegation |
| `tests/checked_arithmetic_test.rs` | throughout | Ceiling-breach and ordinary-sum checks |
| `exact_add/src/lib.rs:37,77,112` | — | `money_add`, `price_add`, `money_saturating_add` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::checked_add`; exercised by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `money_add`/`price_add`, and the checked attempt `money_saturating_add` clamps on failure |

## Caller Tree

- [Qty::checked_add](016_checked_add_qty.md) (`src/lib.rs:490`)
- **External:** `exact_add::money_add` (`exact_add/src/lib.rs:37`), `price_add` (`:77`), `money_saturating_add` (`:112`)

## Callee Tree

- **External:** `i64::checked_add` — `self.minor.checked_add( rhs.minor )`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:238`)
