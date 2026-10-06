# 004: money_eq

## Representation

Whether two money values are equal, dispatching to `Money`'s own derived
`PartialEq`. The crate's only equality function — no `qty_eq` or `price_eq`
exists, verified via exhaustive grep of this crate's own source; equality
for `Quantity`/`Price` has no dedicated free function here today.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:57`

```rust
#[ must_use ]
pub fn money_eq( a : Money, b : Money ) -> bool
{
  a == b
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 57 | Declaration |
| `tests/cmp_test.rs` | 39-40 | Equal pair and unequal pair, one assertion each |
| `exact_arith/src/lib.rs:137` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_cmp` | `(defining crate)` | Exercised by its own equality test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name (`exact_arith/src/lib.rs:137`); no production call
site exists anywhere else in the workspace, confirmed via
`grep -rn 'money_eq(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Money::eq` (via `a == b`, `Money`'s own derived `PartialEq`)
