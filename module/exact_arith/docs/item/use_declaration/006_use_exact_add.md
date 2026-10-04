# 006: pub use exact_add::{ ... }

## Representation

Re-exports `exact_add`'s 9 checked/saturating arithmetic functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:89-100`

```rust
pub use exact_add::
{
  money_add,
  money_checked_neg,
  money_saturating_add,
  money_sub,
  price_add,
  price_sub,
  qty_add,
  qty_saturating_add,
  qty_sub,
};
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 89-100 | Declaration |

Confirmed via a full-workspace grep: not one of these 9 names is imported
or called through `exact_arith` anywhere, including this crate's own test
suite — one of 6 re-export blocks in this facade with zero confirmed usage
at any distance (see readme Notable Findings for the full 3-way split).
`exact_conserve` calls `exact_add::money_add`/`qty_add` directly as a crate
dependency, not through this facade.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
