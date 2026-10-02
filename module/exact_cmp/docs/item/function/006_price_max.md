# 006: price_max

## Representation

The greater of two price values, dispatching to `Price`'s own derived `Ord`
via `core::cmp::Ord::max`. Paired with [price_min](005_price_min.md) as the
crate's only min/max functions — `Price`-only, same as that function.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:59`

```rust
#[ must_use ]
pub fn price_max( a : Price, b : Price ) -> Price
{
  a.max( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 59 | Declaration |
| `tests/cmp_test.rs` | 51-52 | Both argument orderings, same expected result |
| `exact_arith/src/lib.rs:119` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_cmp` | `(defining crate)` | Exercised by its own min/max test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name (`src/lib.rs:119`); no production call
site exists anywhere else in the workspace, confirmed via
`grep -rn 'price_max(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Decimal::max` (via `a.max( b )`, `Price`'s own derived `Ord`)
