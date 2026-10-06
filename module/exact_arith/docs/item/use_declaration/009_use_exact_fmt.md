# 009: pub use exact_fmt::{ ... }

## Representation

Re-exports `exact_fmt`'s error type, its one non-allocating render primitive,
and the 3 per-kind `to_string()` wrappers.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:118`

```rust
pub use exact_fmt::{ FmtError, fmt_into, money_fmt, price_fmt, qty_fmt };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 118 | Declaration |

Confirmed via a full-workspace grep: not one of these 5 names is imported
or called through `exact_arith` anywhere, including this crate's own test
suite. Every real consumer renders via `{}`/`.to_string()` on a `Money`/
`Quantity`/`Price` value directly (its own inherent `Display`), never
through this facade-level convenience wrapper.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
