# 001: money_cmp

## Representation

Compare two money values, dispatching to `Money`'s own derived `Ord`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_cmp/src/lib.rs:36`

```rust
#[ must_use ]
pub fn money_cmp( a : Money, b : Money ) -> core::cmp::Ordering
{
  a.cmp( &b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 36 | Declaration |
| `tests/cmp_test.rs` | 14-16 | Less/Greater/Equal, all three orderings against the same pair |
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
`grep -rn 'money_cmp(' --include='*.rs' substrate/ module/` across the full
tree, excluding `/target/`.

## Callee Tree

- **External:** `exact_kind::Money::cmp` (via `a.cmp( &b )`, `Money`'s own derived `Ord`)
