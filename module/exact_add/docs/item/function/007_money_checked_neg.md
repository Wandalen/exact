# 007: money_checked_neg

## Representation

Negate a money value. Money is signed, so negation is always allowed (unlike
`Quantity`, which has no negation function at all — a negative quantity is
unrepresentable, so there is nothing to dispatch to).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:95`

```rust
pub const fn money_checked_neg( a : Money ) -> Result< Money, KindError >
{
  a.checked_neg()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 95 | Declaration |
| `tests/checked_and_saturating_add_test.rs:50-51` | — | Negation at both signs; double negation round-trips to the original |
| `exact_arith/src/lib.rs:86` | — | Facade re-export |

No production (non-test) file outside `exact_add` calls `money_checked_neg`
— an honest empty finding. It is also, per the crate's own module doc
comment, the sole reason `NegNotAllowed` is unreachable and dropped from
this crate's error surface: the only negation exposed here is over `Money`,
a kind that is already signed.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own double-negation test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own test — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Decimal::checked_neg` (via `a.checked_neg()`, `Money` being an alias for `Decimal< SCALE >`)
