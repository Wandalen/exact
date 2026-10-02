# exact_snap

Tier 2 — snapping a price or a quantity onto a grid: `Tick` for price,
`Lot` for quantity. Net-new: no real precedent exists for either structure
or for snapping itself.

```rust
use exact_kind::Price;
use exact_round::Rounding;
use exact_snap::{ Tick, price_snap_tick };

let tick = Tick::new( Price::from_minor( 5 ).unwrap() ).unwrap();
let between = Price::from_minor( 17 ).unwrap();
assert_eq!( price_snap_tick( between, tick, Rounding::Down ).unwrap().minor(), 15 );
```

## Why it shares `exact_round::round_div`

Snapping is "divide by the grid spacing, round, multiply back" — the same
division `exact_ratio::div_round` needs. Both crates already depend on
`exact_round` for `Rounding` itself, so the actual division logic lives
there once, as `round_div`, rather than being hand-written twice under two
different names.

## What it does not do

`Tick::new` and `Lot::new` refuse only an exactly-zero grid spacing — a
negative one is accepted, since `round_div`'s own divisor normalization
already handles it correctly, and the preferred design names no error for
it.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` and `exact_round` |
| [`src/lib.rs`](src/lib.rs) | `Tick`, `Lot`, `price_snap_tick`, `qty_snap_lot`, `SnapError` |
| [`tests/snap_test.rs`](tests/snap_test.rs) | Construction refusal, on-grid identity, and every rounding mode |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/workaround/`](docs/workaround/readme.md) | External constraints this crate absorbs — none |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the `Price`/`Quantity` types this crate snaps
- [`exact_round/`](../exact_round/readme.md) — the shared `round_div` this crate's snap is built from
- [`exact_ratio/`](../exact_ratio/readme.md) — the sibling tier-2 crate sharing the same `round_div`
