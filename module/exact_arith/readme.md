# exact_arith

Tier 4 — the facade over this family's 14-crate value substrate. A single
block of `pub use` per source crate, re-exporting everything and adding
nothing, so a consumer depends on one crate instead of fourteen.

Evaluating this as a dependency against an existing decimal/money crate?
See [`../../readme.md`](../../readme.md) § Why Not an Existing Crate for the
reasons, and [`../../docs/research/001_exact_vs_open_source_alternatives.md`](../../docs/research/001_exact_vs_open_source_alternatives.md)
for the full sourced comparison against `rust_decimal`, `bigdecimal`,
`fastnum`, and others.

```rust
use exact_arith::{ Money, Price, Quantity, Rounding, price_mul_qty };

let price = Price::parse( "1.25" ).unwrap();
let held = Quantity::parse( "2.5" ).unwrap();
assert_eq!( price_mul_qty( price, held, Rounding::HalfEven ).unwrap(), Money::parse( "3.125" ).unwrap() );
```

## Why no `exact_zero_money`/`exact_zero_qty`/`exact_zero_price`

The preferred design's type doc names these as the only functions this
facade would define itself. Each would be a one-line duplicate of a constant
already re-exported (`Money::ZERO`, `Quantity::ZERO`, `Price::ZERO`) — the
first crack in the real `exact_arithmetic` facade's own mechanically-tested
"declares nothing of its own" discipline, ported here verbatim including its
purity test. See the disclosed deviation in [`src/lib.rs`](src/lib.rs)'s
module doc comment.

## History

This crate replaced `exact_arithmetic` — the family's earlier, 3-crate-wide
facade — at the 15-crate migration's cutover. Same role (the one dependency
every consumer takes), wider surface (14 leaves instead of 3). The old crate
no longer exists on disk; its content is still retrievable with
`git show HEAD:exact_arithmetic/readme.md` (and likewise for
any other path under it) from before the cutover commit.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on all 14 other crates in this family |
| [`src/lib.rs`](src/lib.rs) | `pub use` of every public item from all 14 leaves — nothing else |
| [`tests/facade_test.rs`](tests/facade_test.rs) | End-to-end settlement through the facade alone, representative name resolution, and the mechanical "declares nothing" purity test |
| [`tests/no_alloc_test.rs`](tests/no_alloc_test.rs) | Rendering into a stack buffer and both `*_dust_split_into` functions make no heap allocation, counted by the `assert_no_alloc` dev-dependency's allocator (no `unsafe` in this crate) |
| [`tests/bench_vs_f64.rs`](tests/bench_vs_f64.rs) | Hard problem 12 / feature 22's proposed add-compare-ratio timing comparison against `f64`, informational only |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Design documentation — feature scope, the facade-purity invariant, module index, workaround (none) |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |
