# 005: pub use exact_kind::{ ... }

## Representation

Re-exports the family's core conserved-value types — the generic `Decimal`,
`Qty` and `PriceAt`, their standard-scale aliases `Money`, `Quantity` and
`Price`, their shared `KindError`, and the one struct the three are aliases
of, `Amount< K, SCALE >`, with its sealed `Kind` trait and the three kinds
`MoneyKind`, `QtyKind` and `PriceKind` (→ [`exact_kind`'s ADR-006](../../../../exact_kind/docs/decisions/006_one_amount_struct_generic_over_kind.md)). The most widely
consumed re-export block in this entire facade by a wide margin.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:88`

```rust
pub use exact_kind::{ Amount, Decimal, Kind, KindError, Money, MoneyKind, Price, PriceAt, PriceKind, Qty, QtyKind, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 88 | Declaration |
| `src/lib.rs` | 21-34 | Module-level doc-test — `Money`, `Quantity` |
| `tests/facade_test.rs` | throughout | `Money`, `Quantity`, `KindError` used in both tests |
| `tests/no_alloc_test.rs` | 14,43-45,89-92 | `Money`, `Quantity`, `Price` rendered and split without allocating |
| `tests/any_scale_test.rs` | throughout | `Decimal`, `PriceAt`, `Qty` at scales 2 and 8 |
| `exchange_core/src/lib.rs:63-66` | — | **Production** — `KindError`, `Money`, `Quantity` re-exported one layer further |
| `exchange_book`, `exchange_match`, `exchange_escrow`, `exchange_types` | `src/lib.rs` each | **Production** — `Money`/`Quantity` (every one of the 5 `substrate/exchange/` crates) |
| `exchange_match/src/lib.rs:70` | — | **Production** — `KindError` specifically |
| `cluster_economy` | `src/lib.rs` | **Production** — `Money`/`Quantity` |
| `smoke_cluster_economy_market`, `smoke_module_cluster_integration` | `src/lib.rs` each | **Demo-lane** — `Money`/`Quantity` (grading lanes per their own module doc comments: "Headless smoke lane grading...") |

`Decimal` and `Qty` themselves (the generic aliases, as opposed to
the `Money`/`Quantity`/`Price` aliases at the standard scale) have no confirmed caller anywhere
outside `exact_kind`'s own tests and this facade's declaration — every real
consumer, inside and outside `module/`, reaches the family
exclusively through the 3 named aliases. `Amount`, `Kind` and the three
kinds have no caller outside `exact_kind` either: they are re-exported
because this facade re-exports every public item, so a name that appears in
`Decimal`'s own type — `Amount< MoneyKind, SCALE >` in a compiler message or
`rustdoc` — can be named through the facade too.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs`, `tests/no_alloc_test.rs`, `tests/any_scale_test.rs` | **Exercised** |
| `exchange_core` | `src/lib.rs` | **Production** — re-exported one layer further |
| `exchange_book` | `src/lib.rs` | **Production** |
| `exchange_match` | `src/lib.rs` | **Production** |
| `exchange_escrow` | `src/lib.rs` | **Production** |
| `exchange_types` | `src/lib.rs` | **Production** |
| `cluster_economy` | `src/lib.rs` | **Production** |
| `smoke_cluster_economy_market` | `src/lib.rs` | **Demo-lane** |
| `smoke_module_cluster_integration` | `src/lib.rs` | **Demo-lane** |
