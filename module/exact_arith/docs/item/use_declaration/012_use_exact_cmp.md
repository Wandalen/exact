# 012: pub use exact_cmp::{ ... }

## Representation

Re-exports `exact_cmp`'s 6 comparison/equality/min-max functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:125`

```rust
pub use exact_cmp::{ money_cmp, money_eq, price_cmp, price_max, price_min, qty_cmp };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 125 | Declaration |

Confirmed via a full-workspace grep: not one of these 6 names is imported
or called through `exact_arith` anywhere, including this crate's own test
suite. Every real comparison in this workspace uses `Money`/`Quantity`/
`Price`'s own derived `Ord`/`PartialEq` directly (`a.cmp(&b)`, `a == b`,
`a.min(b)`), never this facade-level named-function wrapper — matching
`exact_cmp`'s own catalog finding that all 6 functions have zero production
callers even within `module/` itself.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
