# 009: qty_saturating_add

## Representation

Add two quantities, clamping to the declared ceiling instead of refusing.
Only the upper bound can ever clamp — two non-negative quantities never sum
below zero — so, unlike [`money_saturating_add`](008_money_saturating_add.md),
there is no sign check and no dependency on `exact_sign`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:142`

```rust
pub const fn qty_saturating_add( a : Quantity, b : Quantity ) -> Quantity
{
  match a.checked_add( b )
  {
    Ok( sum ) => sum,
    Err( _ ) => Quantity::MAX,
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 142 | Declaration |
| `tests/checked_and_saturating_add_test.rs:78,82` | — | Clamps at the ceiling; matches checked addition in range |
| `exact_arith/src/lib.rs:98` | — | Facade re-export |

No production (non-test) file outside `exact_add` calls `qty_saturating_add`
— an honest empty finding.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own clamp-boundary and in-range-parity tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own tests — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Qty::checked_add` (via `a.checked_add( b )`)
