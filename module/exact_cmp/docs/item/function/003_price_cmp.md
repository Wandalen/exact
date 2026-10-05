# 003: price_cmp

## Representation

Compare two price values, dispatching to `Price`'s own derived `Ord`.
`Price`'s `Ord` compares the `Money` it wraps, so this orders prices exactly
as [money_cmp](001_money_cmp.md) orders money.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:50`

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
| `src/lib.rs` | 50 | Declaration |
| `tests/cmp_test.rs` | 29 | Less ordering, alongside the sibling `qty_cmp` check in the same test |
| `exact_arith/src/lib.rs:125` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_cmp` | `(defining crate)` | Exercised by its own ordering test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name (`exact_arith/src/lib.rs:125`); no production call
site exists anywhere else in the workspace, confirmed via
`grep -rn 'price_cmp(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Price`'s derived `Ord::cmp` (via `a.cmp( &b )`), comparing the wrapped `Money`
