# exact_dust

Tier 3 — splitting a value into equal or weighted parts under a chosen rounding mode,
giving the remainder ("dust") an explicit destination instead of letting it
silently vanish into truncation.

```rust
use exact_dust::{ money_dust_split, DustTo };
use exact_kind::Money;
use exact_round::Rounding;

let total = Money::from_minor( 11 ).unwrap();
let shares = money_dust_split( total, 4, Rounding::Down, DustTo::First ).unwrap();
assert_eq!( shares[ 0 ], Money::from_minor( 5 ).unwrap() ); // 2 + the 3-unit remainder
assert_eq!( shares[ 1 ], Money::from_minor( 2 ).unwrap() );
```

## Why `exact_round`, not `exact_ratio`

The preferred design lists `exact_kind, exact_ratio` as this crate's
dependencies, but the actual need is the integer-count, mode-driven division
`exact_snap` already depends on `exact_round` directly for — not
`exact_ratio`'s rational-multiplier surface, none of which this crate uses.
See the disclosed deviation in [`src/lib.rs`](src/lib.rs)'s module doc
comment.

## Why no `price_dust_split`

`exact_ratio` already set this precedent by shipping `money_div_round`/
`qty_div_round` with no `price_div_round` — splitting a total among parties
or a holding into lots are real operations for `Money` and `Quantity`;
splitting a *price* into equal shares has no natural reading and no consumer
anywhere in this codebase.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` and `exact_round` |
| [`src/lib.rs`](src/lib.rs) | `DustTo`, `DustError`, `money`/`qty_dust_split`, `_into`, `_remainder`, and `money_dust_split_weighted` |
| [`tests/dust_split_test.rs`](tests/dust_split_test.rs) | Even splits, remainder destinations, the non-negative kind's own negative-slot refusal under `Up`, and `Exact`'s refusal of an uneven split |
| [`tests/weighted_split_test.rs`](tests/weighted_split_test.rs) | Weighted splits: exact percentages, the leftover routed by each `DustTo`, `Exact`'s refusal, `Reject` refusing a remainder but not a rounded share, empty and zero weights, every part on the total's side under `TowardZero`, equal weights matching the equal split, no overflow at `Money::MAX`, and a conservation sweep — 70 035 cases, 2 001 totals × 5 weight lists × 7 rounding modes, the parts always summing to the total under `First` and differing only by the dust under `Sink` |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Algorithm, decisions, and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the conserved value types this crate splits
- [`exact_round/`](../exact_round/readme.md) — the shared `round_div` this crate's per-share division is built from
