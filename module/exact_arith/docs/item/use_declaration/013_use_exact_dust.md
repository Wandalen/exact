# 013: pub use exact_dust::{ ... }

## Representation

Re-exports `exact_dust`'s `DustTo` policy enum, its error enum, and all 6
split/remainder functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:127`

```rust
pub use exact_dust::{ DustError, DustTo, money_dust_remainder, money_dust_split, money_dust_split_into, qty_dust_remainder, qty_dust_split, qty_dust_split_into };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 127 | Declaration |
| `src/lib.rs` | 21,32 | Module-level doc-test — `money_dust_split`, `DustTo` |
| `tests/facade_test.rs:9,38` | — | `money_dust_split`, `DustTo` round-tripped |
| `smoke_exact_market_split/src/lib.rs:45` | — | **Demo-lane** — `money_dust_split`, `DustTo`, per `exact_dust`'s own catalog (`function/001_money_dust_split.md`) |

**Only `money_dust_split`/`DustTo` of the 8 re-exported names are exercised
anywhere** — `money_dust_split_into`, both `*_remainder` functions, `DustError`,
and the entire `qty_*` side (`qty_dust_split`, `qty_dust_split_into`,
`qty_dust_remainder`) have no confirmed caller through this facade,
confirmed via a full-workspace grep.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | **Partially exercised** — `money_dust_split`/`DustTo` only |
| `smoke_exact_market_split` | `src/lib.rs` | **Demo-lane** |
