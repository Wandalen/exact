# 002: Decimal::ZERO

## Representation

Zero — the one infallible constructor, representable at every scale.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:179`

```rust
pub const ZERO : Self = Self { minor : minor_zero() };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 179,420 | Declaration; `Qty::ZERO`'s own definition wraps this value |
| `tests/checked_arithmetic_test.rs` | throughout | Accumulator seed, boundary comparisons |
| `exact_dust/tests/dust_split_test.rs:80` | — | Array-fill seed `[ Money::ZERO; 4 ]` |
| `exact_arith/src/lib.rs:41,45` (doc) | — | Facade doc comment naming it as part of the re-exported surface |
| `exact_arith/tests/facade_test.rs:39` | — | `try_fold` accumulator seed |
| `smoke_exact_market_split/src/lib.rs:58,206` | — | **Production** — accumulator seed in the ledger's running total and the recombination check |
| `smoke_exact_market_split/tests/lane_test.rs:128` | — | `try_fold` accumulator seed |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::ZERO`; exercised by its own tests |
| `exact_dust`, `exact_arith` | `tests/*.rs` | Test-only accumulator/array seed |
| `smoke_exact_market_split` | `src/lib.rs`, `tests/lane_test.rs` | **Production** — the ledger's running-total accumulator starts here |
