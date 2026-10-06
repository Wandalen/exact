# 014: pub use exact_conserve::{ ... }

## Representation

Re-exports `exact_conserve`'s posting/report types, its error enum, the
untyped-log auditor, and the 4 typed per-kind convenience functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:141`

```rust
pub use exact_conserve::{ ConservationError, Entry, Report, money_conserve_into, money_sum_assert_zero, qty_conserve_into, qty_sum_assert_zero, verify };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 141 | Declaration |
| `src/lib.rs` | 21,30 | Module-level doc-test — `Entry`, `verify` (return value used via `.is_balanced()`, `Report` not bound by name) |
| `tests/facade_test.rs:9-10,28-30,61` | — | `Entry`, `Report`, `verify`, `ConservationError` all exercised — the only one of the 14 re-export blocks whose `Display` impl is asserted by exact rendered text (`ConservationError::Overflow.to_string()`) |
| `cluster_economy/src/market.rs:4,507-508,518-519` | — | **Production** — `verify`/`is_balanced` gate a settlement's cash and asset legs; `Entry` built via `Entry::new` at lines 504-505,515-516; imported `use exact_arith::{ Entry, Money, Quantity, verify };` |
| `exchange_core/src/lib.rs:63-66,460-461` | — | **Production** — re-exports `ConservationError`/`Entry`/`Report`/`verify` one hop further to its own consumers, and calls `Entry::new` directly in its own settlement path |
| `exchange_core/tests/submission_test.rs:257-258` | — | Integration test — `verify`/`is_balanced` |
| `cluster_economy/tests/economy_test.rs:255-264` | — | Reconciliation assertions |
| `smoke_exchange_core/src/lib.rs:105,107` | — | Demo-lane settlement audit |
| `smoke_exact_market_split/src/lib.rs:127-128,308-309,313-314`, `tests/lane_test.rs:60-61,63-64` | — | Demo-lane ledger checks, including the ported `bug_reproducer` regression test |

**`Entry`, `Report`, `verify`, and `ConservationError` are the facade's most
heavily downstream-used re-export block** — per `exact_conserve`'s own
catalog (`function/001_verify.md`, `associated_function/001_new_entry.md`,
`003_is_balanced.md`), confirmed via a full-workspace grep, not a
dependents-only one (the lesson that same catalog's first draft learned the
hard way, see its readme's Notable Findings). The 4 typed convenience
functions — `money_conserve_into`, `qty_conserve_into`,
`money_sum_assert_zero`, `qty_sum_assert_zero` — have the opposite profile:
zero callers anywhere outside `exact_conserve`'s own tests, not even this
facade's own test suite, matching that catalog's own finding for all four.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | **Partially exercised** — `Entry`/`Report`/`verify`/`ConservationError` only; the 4 typed convenience functions are declared but untouched |
| `cluster_economy` | `src/market.rs` | **Production** — settlement-path conservation gate |
| `exchange_core` | `src/lib.rs`, `tests/submission_test.rs` | **Production** re-export, plus integration test |
| `smoke_exchange_core` | `src/lib.rs` | Demo-lane settlement audit |
| `smoke_exact_market_split` | `src/lib.rs`, `tests/lane_test.rs` | Demo-lane ledger checks |
