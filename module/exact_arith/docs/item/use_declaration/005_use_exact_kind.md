# 005: pub use exact_kind::{ ... }

## Representation

Re-exports the family's core conserved-value types — `Decimal`, `Money`,
`Price`, `Quantity`, `Qty`, and their shared `KindError`. The most widely
consumed re-export block in this entire facade by a wide margin.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:88`

```rust
pub use exact_kind::{ Decimal, KindError, Money, Price, Qty, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 88 | Declaration |
| `src/lib.rs` | 21-34 | Module-level doc-test — `Money`, `Quantity` |
| `tests/facade_test.rs` | throughout | `Money`, `Quantity`, `KindError` used in both tests |
| `tests/no_alloc_test.rs` | 14,43-45,66-69 | `Money`, `Quantity`, `Price` rendered and split without allocating |
| `exchange_core/src/lib.rs:63-66` | — | **Production** — `KindError`, `Money`, `Quantity` re-exported one layer further |
| `exchange_book`, `exchange_match`, `exchange_escrow`, `exchange_types` | `src/lib.rs` each | **Production** — `Money`/`Quantity` (every one of the 5 `substrate/exchange/` crates) |
| `exchange_match/src/lib.rs:70` | — | **Production** — `KindError` specifically |
| `cluster_economy` | `src/lib.rs` | **Production** — `Money`/`Quantity` |
| `smoke_cluster_economy_market`, `smoke_module_cluster_integration` | `src/lib.rs` each | **Demo-lane** — `Money`/`Quantity` (grading lanes per their own module doc comments: "Headless smoke lane grading...") |

`Decimal` and `Qty` themselves (the generic backing types, as opposed to
the `Money`/`Quantity` aliases and the `Price` struct) have no confirmed caller anywhere
outside `exact_kind`'s own tests and this facade's declaration — every real
consumer, inside and outside `module/`, reaches the family
exclusively through the 3 named aliases.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs`, `tests/no_alloc_test.rs` | **Exercised** |
| `exchange_core` | `src/lib.rs` | **Production** — re-exported one layer further |
| `exchange_book` | `src/lib.rs` | **Production** |
| `exchange_match` | `src/lib.rs` | **Production** |
| `exchange_escrow` | `src/lib.rs` | **Production** |
| `exchange_types` | `src/lib.rs` | **Production** |
| `cluster_economy` | `src/lib.rs` | **Production** |
| `smoke_cluster_economy_market` | `src/lib.rs` | **Demo-lane** |
| `smoke_module_cluster_integration` | `src/lib.rs` | **Demo-lane** |
