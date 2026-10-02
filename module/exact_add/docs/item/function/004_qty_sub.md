# 004: qty_sub

## Representation

Subtract two quantities, refusing to go below zero — dispatching straight to
`exact_kind`'s own checked subtraction, which already carries the
non-negativity refusal.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:65`

```rust
pub const fn qty_sub( a : Quantity, b : Quantity ) -> Result< Quantity, KindError >
{
  a.checked_sub( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 65 | Declaration |
| `tests/checked_and_saturating_add_test.rs:31` | — | Non-negativity refusal, subtracting a larger quantity from a smaller one |
| `exact_arith/src/lib.rs:93` | — | Facade re-export |

No production (non-test) file outside `exact_add` calls `qty_sub` — an
honest empty finding. `exact_conserve` only ever adds legs via
[`qty_add`](003_qty_add.md); it never subtracts.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own non-negativity-refusal test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own test — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Qty::checked_sub` (via `a.checked_sub( b )`, `Quantity` being an alias for `Qty< SCALE >`)
