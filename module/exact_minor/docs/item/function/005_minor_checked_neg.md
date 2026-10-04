# 005: minor_checked_neg

## Representation

Negate a count of minor units, refusing the one backing value that cannot
negate, `Backing::MIN`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:270`

```rust
pub const fn minor_checked_neg( a : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_neg()
  {
    Some( neg ) => Ok( Minor( neg ) ),
    None => Err( MinorError::Overflow { operation : "neg" } ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 270-277 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Round-trip negation over `[0, 1, -1, Backing::MAX]`, plus the `Backing::MIN` refusal |
| `exact_kind/src/lib.rs:296` | — | **Production** — `Decimal::checked_neg` |
| `exact_arith/src/lib.rs:71` | — | Facade re-export only |

`exact_kind`'s `Decimal::checked_neg` delegates here on its stored `Minor`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own round-trip and boundary tests |
| `exact_kind` | `src/lib.rs` | **Production** — `Decimal::checked_neg`, behind every `Money`/`Price`/`Quantity` neg |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_kind::Decimal::checked_neg` (`exact_kind/src/lib.rs:296`)

## Callee Tree

- **External:** `i64::checked_neg` — `a.checked_neg()`
