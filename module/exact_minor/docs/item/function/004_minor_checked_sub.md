# 004: minor_checked_sub

## Representation

Subtract two counts of minor units, refusing a difference that leaves the
backing width.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:254`

```rust
pub const fn minor_checked_sub( a : Minor, b : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_sub( b.0 )
  {
    Some( diff ) => Ok( Minor( diff ) ),
    None if b.0 < 0 => Err( MinorError::Overflow { operation : "sub" } ),
    None => Err( MinorError::Underflow { operation : "sub" } ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 254-262 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Ordinary difference and `Backing::MIN`-boundary refusal |
| `exact_kind/src/lib.rs:262` | — | **Production** — `Decimal::checked_sub` |
| `exact_arith/src/lib.rs:73` | — | Facade re-export only |

`exact_kind`'s `Decimal::checked_sub` delegates here on its stored `Minor`,
as [minor_checked_add](003_minor_checked_add.md) does for addition.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own boundary tests |
| `exact_kind` | `src/lib.rs` | **Production** — `Decimal::checked_sub`, behind every `Money`/`Price`/`Quantity` sub |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_kind::Decimal::checked_sub` (`exact_kind/src/lib.rs:262`)

## Callee Tree

- **External:** `i64::checked_sub` — `a.checked_sub( b )`
