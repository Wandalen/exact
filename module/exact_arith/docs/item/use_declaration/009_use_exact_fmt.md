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
| `tests/no_alloc_test.rs` | 14,51,53-54,57 | `fmt_into` renders every kind into a stack buffer, checked to allocate nothing |

Confirmed via a full-workspace grep: of these 5 names only `fmt_into` is
called through `exact_arith`, and only by this crate's own
`tests/no_alloc_test.rs`. Every real consumer renders via `{}`/`.to_string()` on a `Money`/
`Quantity`/`Price` value directly (its own inherent `Display`), never
through this facade-level convenience wrapper.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)`, `tests/no_alloc_test.rs` | Declared; `fmt_into` exercised by this crate's own allocation test |
