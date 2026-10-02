# 003: price_cmp

## Representation

Compare two price values, dispatching to `Price`'s own derived `Ord`.
`Price` is `Money` under `exact_kind`'s disclosed deviation, so this takes
the identical path as [money_cmp](001_money_cmp.md) through a distinct name.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:38`

```rust
#[ must_use ]
pub fn price_cmp( a : Price, b : Price ) -> core::cmp::Ordering
{
  a.cmp( &b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 38 | Declaration |
| `tests/cmp_test.rs` | 29 | Less ordering, alongside the sibling `qty_cmp` check in the same test |
| `exact_arith/src/lib.rs:119` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_cmp` | `(defining crate)` | Exercised by its own ordering test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name (`src/lib.rs:119`); no production call
site exists anywhere else in the workspace, confirmed via
`grep -rn 'price_cmp(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Decimal::cmp` (via `a.cmp( &b )`, `Price` being an alias for `Decimal< SCALE >`)
