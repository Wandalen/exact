# exact

Workstream 006's exact arithmetic: decimals, quantities, and the conservation audit,
decomposed into the 15-crate target design, with `exact_arith` as a deliberately thin
facade — a single block of `pub use` over the 14 leaf crates — plus one smoke lane.

This directory holds only the crates themselves. Each crate's purpose and the
real dependency graph are in
[`docs/guide/001_crate_family_overview.md`](docs/guide/001_crate_family_overview.md)
— not duplicated here. The original proposal's own dependency tree, and where
the real family deviates from it, is recorded in
[`docs/readme.md`](docs/readme.md) § Dependency Tree.

To see the whole family work end to end, run the one demo:
`cargo run -p smoke_exact_market_split` — headless, no book, no wallets, prints a
golden result proving `0.1 + 0.2` stays exact.

## Why Not an Existing Crate

- Money, quantity, and price must stay non-interchangeable types
- Cross-scale operations must fail to compile, not resolve silently at runtime
- Values must be plain `Copy` bits, snapshot-safe, no heap
- Needs a built-in zero-sum conservation audit
- Dust remainder and tick/lot snapping are first-class policy
- Bit-for-bit determinism required across every platform
- Hot match-loop operations (add, compare, ratio) must be cheap — no heap allocation
- No single existing crate bundles all of these

Full comparison against `rust_decimal`, `bigdecimal`, `fixed`, and others:
[`docs/research/001_exact_vs_open_source_alternatives.md`](docs/research/001_exact_vs_open_source_alternatives.md).

## Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| [`exact_minor/`](module/exact_minor/readme.md) | Tier 0 of the 15-crate target design — the raw subunit integer and its checked/saturating arithmetic |
| [`exact_scale/`](module/exact_scale/readme.md) | Tier 0 of the 15-crate target design — scale-factor math and the declared ceiling |
| [`exact_round/`](module/exact_round/readme.md) | Tier 0 of the 15-crate target design — rounding modes and the family's default policy |
| [`exact_sign/`](module/exact_sign/readme.md) | Tier 1 of the 15-crate target design — sign classification and the negative-value admission policy |
| [`exact_kind/`](module/exact_kind/readme.md) | Tier 1 of the 15-crate target design — the conserved value type family: `Money`, `Qty`, `Price` |
| [`exact_add/`](module/exact_add/readme.md) | Tier 2 of the 15-crate target design — checked and saturating add/sub per kind |
| [`exact_ratio/`](module/exact_ratio/readme.md) | Tier 2 of the 15-crate target design — a rational multiplier and mode-driven division per kind |
| [`exact_parse/`](module/exact_parse/readme.md) | Tier 2 of the 15-crate target design — text parsing per kind |
| [`exact_fmt/`](module/exact_fmt/readme.md) | Tier 2 of the 15-crate target design — formatting per kind and a buffer-writing primitive |
| [`exact_bytes/`](module/exact_bytes/readme.md) | Tier 2 of the 15-crate target design — the wire encoding and its to/from conversions per kind |
| [`exact_snap/`](module/exact_snap/readme.md) | Tier 2 of the 15-crate target design — tick/lot grid snapping |
| [`exact_cmp/`](module/exact_cmp/readme.md) | Tier 2 of the 15-crate target design — comparison, equality, and min/max per kind |
| [`exact_dust/`](module/exact_dust/readme.md) | Tier 3 of the 15-crate target design — equal-parts splitting with an explicit remainder destination |
| [`exact_conserve/`](module/exact_conserve/readme.md) | Tier 3 of the 15-crate target design — conservation auditing, the plain-log verifier plus a typed per-kind layer |
| [`exact_arith/`](module/exact_arith/readme.md) | Tier 4 of the 15-crate target design — the facade over all 14 other crates, re-exports only |
| [`smoke_exact_market_split/`](module/smoke_exact_market_split/readme.md) | Tier 5 of the 15-crate target design — this family's slice end to end through `exact_arith` alone |
| [`docs/`](docs/readme.md) | Family-level material belonging to no single crate — two archived external conversations, plus the 15-crate target design corpus |
| [`verb/`](verb/readme.md) | do-protocol verb scripts — leveled test, lint, build, doc, clean, and per-crate dispatch, mirroring `../ring/verb/` |
| [`.github/`](.github/workflows/ci.yml) | CI — nextest, doctests, clippy, and a clean rustdoc build on every push/PR |
