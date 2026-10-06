# 003: pub use exact_round::{ ... }

## Representation

Re-exports `exact_round`'s rounding-mode vocabulary and its two division
functions, `round_div` and `round_div_wide` (its `i128` counterpart) — the
third Tier-0 root.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:84`

```rust
pub use exact_round::{ Rounding, RoundError, round_div, round_div_wide, rounding_default, rounding_name };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 84 | Declaration |
| `src/lib.rs` | 21,33 | Module-level doc-test — `Rounding` |
| `tests/facade_test.rs:10,38,56` | — | `Rounding`, `round_div` both exercised |
| `smoke_exact_market_split/src/lib.rs:45` | — | **Demo-lane** — `Rounding` |

`RoundError` and the 2 convenience functions (`rounding_default`,
`rounding_name`) have no confirmed caller anywhere through this facade — the
same "re-exported but never called even through the defining crate itself"
finding already recorded in `exact_round`'s own catalog, now confirmed to
also hold at the facade layer.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | **Exercised** — doc-test and integration test both use `Rounding`/`round_div` |
| `smoke_exact_market_split` | `src/lib.rs` | Demo-lane `Rounding` usage |
