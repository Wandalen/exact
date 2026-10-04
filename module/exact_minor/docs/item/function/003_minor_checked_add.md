# 003: minor_checked_add

## Representation

Add two counts of minor units, refusing a sum that leaves the backing width.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:238`

```rust
pub const fn minor_checked_add( a : Minor, b : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_add( b.0 )
  {
    Some( sum ) => Ok( Minor( sum ) ),
    None if b.0 > 0 => Err( MinorError::Overflow { operation : "add" } ),
    None => Err( MinorError::Underflow { operation : "add" } ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 238-246 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Ordinary sum and `Backing::MAX`-boundary refusal |
| `exact_kind/src/lib.rs:248` | — | **Production** — `Decimal::checked_add` |
| `exact_arith/src/lib.rs:70` | — | Facade re-export only |

`exact_kind`'s `Decimal::checked_add` delegates here on its stored `Minor`,
so the overflow check is implemented once, in this crate.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own boundary tests |
| `exact_kind` | `src/lib.rs` | **Production** — `Decimal::checked_add`, behind every `Money`/`Price`/`Quantity` add |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_kind::Decimal::checked_add` (`exact_kind/src/lib.rs:248`)

## Callee Tree

- **External:** `i64::checked_add` — `a.checked_add( b )`
