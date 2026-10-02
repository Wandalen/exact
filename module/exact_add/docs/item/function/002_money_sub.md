# 002: money_sub

## Representation

Subtract two money values, dispatching straight to `exact_kind`'s own
checked subtraction.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:45`

```rust
pub const fn money_sub( a : Money, b : Money ) -> Result< Money, KindError >
{
  a.checked_sub( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 45 | Declaration |
| `tests/checked_and_saturating_add_test.rs:19` | — | Round-trips `money_add`'s own result back to the original operand |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

No production (non-test) file outside `exact_add` calls `money_sub` — an
honest empty finding. `exact_conserve`, the crate's one production
dependent, folds legs only via [`money_add`](001_money_add.md); it never
subtracts.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own round-trip test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own test — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Decimal::checked_sub` (via `a.checked_sub( b )`, `Money` being an alias for `Decimal< SCALE >`)
