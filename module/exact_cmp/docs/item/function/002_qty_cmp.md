# 002: qty_cmp

## Representation

Compare two quantities, dispatching to `Quantity`'s own derived `Ord`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:31`

```rust
#[ must_use ]
pub fn qty_cmp( a : Quantity, b : Quantity ) -> core::cmp::Ordering
{
  a.cmp( &b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 31 | Declaration |
| `tests/cmp_test.rs` | 25 | Less ordering, alongside the sibling `price_cmp` check in the same test |
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
`grep -rn 'qty_cmp(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Qty::cmp` (via `a.cmp( &b )`, `Quantity`'s own derived `Ord`)
