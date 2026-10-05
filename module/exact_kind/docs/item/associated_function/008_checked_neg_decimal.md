# 008: Decimal::checked_neg

## Representation

Negate. The overflow check is defensive but currently unreachable through
this type: the only backing value that fails to negate is `Backing::MIN`,
and the declared ceiling keeps every constructible value's magnitude far
below that edge.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:294`

```rust
pub const fn checked_neg( self ) -> Result< Self, KindError >
{
  match minor_checked_neg( self.minor )
  {
    Ok( neg ) => Self::from_minor( minor_to_i64( neg ) ),
    Err( e ) => Err( kind_overflow( e ) ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 294 | Declaration — `Qty` has no `checked_neg` (negating a non-negative value is not a `Qty`-shaped operation) |
| `tests/checked_arithmetic_test.rs` | throughout | Round-trip negate, boundary checks at `MAX`/`MIN` |
| `exact_add/src/lib.rs:115` | — | `money_checked_neg`'s entire body |
| `exact_add/tests/checked_and_saturating_add_test.rs:60` | — | `Money::EPSILON.checked_neg()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own round-trip and boundary tests |
| `exact_add` | `src/lib.rs` | **Production** — `money_checked_neg`'s entire implementation is this one call |

## Caller Tree

- **External:** `exact_add::money_checked_neg` (`exact_add/src/lib.rs:115`)

No intra-crate caller — `Qty` has no `checked_neg`, so nothing in this
crate's own `Qty` wrapper reaches this function.

## Callee Tree

- **External:** `exact_minor::minor_checked_neg` — `minor_checked_neg( self.minor )`, then `minor_to_i64`; a failure goes through the private `kind_overflow`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:298`)
