# 008: pub use exact_parse::{ ... }

## Representation

Re-exports `exact_parse`'s 3 per-kind text-parsing functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:116`

```rust
pub use exact_parse::{ money_from_str, price_from_str, qty_from_str };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 116 | Declaration |

Confirmed via a full-workspace grep: not one of these 3 names is imported
or called through `exact_arith` anywhere, including this crate's own test
suite. Every real consumer of parsing in this workspace calls
`Money::parse`/`Quantity::parse`/`Price::parse` (`exact_kind`'s own inherent
methods) directly instead of this facade-level convenience wrapper — the
same bypass pattern already recorded in `exact_parse`'s own catalog.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
