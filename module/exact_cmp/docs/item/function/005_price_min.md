# 005: price_min

## Representation

The lesser of two price values, dispatching to `Price`'s own derived `Ord`
via `core::cmp::Ord::min`. The crate's only min function — no `money_min` or
`qty_min` exists, verified via exhaustive grep of this crate's own source;
min/max here is `Price`-only today.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:52`

```rust
#[ must_use ]
pub fn price_min( a : Price, b : Price ) -> Price
{
  a.min( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 52 | Declaration |
| `tests/cmp_test.rs` | 49-50 | Both argument orderings, same expected result |
| `exact_arith/src/lib.rs:125` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_cmp` | `(defining crate)` | Exercised by its own min/max test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name (`src/lib.rs:119`); no production call
site exists anywhere else in the workspace, confirmed via
`grep -rn 'price_min(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Decimal::min` (via `a.min( b )`, `Price`'s own derived `Ord`)
