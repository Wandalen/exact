# smoke_exact_market_split

This family's slice run end to end in one process through `exact_arith` — the
14-crate facade — alone, with a floating-point control arm that is
**required to disagree**.

```
cargo run -p smoke_exact_market_split
```

```
smoke_exact_market_split — this family's slice, one process

  parse/render   0.1 -> 0.1 (round-trips exactly)
  exact arm      0.1 x 10 = 1
  control arm    0.1 x 10 = 0.99999999999999989 (f64, wrong by construction)
  the arms disagree, which is what makes this lane a test

  quantity       hold 3, withdraw 5 -> -2000000 minor units is below zero, which this kind cannot hold

  audit, clean   balanced: entries 2, net 0
  audit, leaky   UNBALANCED: entries 2, net cash -1 minor units

  market split   100.000001 into 3 -> [33.333335, 33.333333, 33.333333] (recombines exactly)

  sum=0
  parts=3.33,3.33,3.33 dust=0.01
  tick=1.25 lot=9
  extra=1 overflow=1
  wire=1000
  a=0x7a10e52b936be89f b=0x7a10e52b936be89f
  ok

  per asset      65000.00 x 0.12345679 BTC = 8024.69 (cents x satoshis)
  one scale      the same trade at scale 6 = 8024.7 (a cent off)
  wire           a satoshi quantity carries scale byte 8
  split          0.12345679 BTC into 3 -> [0.04115227, 0.04115226, 0.04115226] (recombines exactly)
  audit          USD and BTC each net to zero, each at its own precision

VERDICT: reached — exact arithmetic holds across the full facade,
         the floating-point control arm does not, a market split conserves,
         and each asset keeps its own precision.
```

## Replaces `smoke_exact_arithmetic`

Ported at the Tier 5 cutover: steps 1 through 4 carry `smoke_exact_arithmetic`'s
exact content forward unchanged (including its fix-documented
`ledger` subtraction bug); step 5 is new — a market fill split with
`exact_dust` and recombined with `exact_conserve`, exercising a pairing
neither crate's own unit tests cover; step 6 is new too — the ten scenes of
[`docs/scene/`](../../docs/scene/readme.md) in their proposed golden-print
shape, every amount at scale 2 through `Decimal< 2 >` — so it prints the
proposal's `3.33`/`0.01` and `1000` line for line. Step 7 is new as well: one
trade priced in cents and sized in satoshis, each at its own scale
([`per_asset_trade`](src/lib.rs)) — encoded, split and settled per asset —
beside the same trade forced onto one shared scale of 6, which the lane
**asserts is a cent off**, the way step 2 asserts the `f64` arm is wrong.

## The control arm

A lane that only runs the exact path proves the exact path does not crash. It
does not prove the path is *exact* — a lane built entirely on `f64` would print
the same cheerful verdict for every other step.

So the central claim is made twice, once through the exact types and once
through `f64`, and the lane **asserts that the two disagree**. If a future
change made the exact path inexact, the arms would agree, the assertion would
fail, and the lane would go red.

`f64` appears in this crate and in no other library source of the family —
only `exact_arith`'s timing bench, a test, also uses it.

## Why one dependency

The manifest names [`exact_arith`](../exact_arith/readme.md) and nothing
else. If any type the slice needs were missing from the facade's re-exports,
this lane would not compile — which is what makes the facade's completeness a
thing the tree tests rather than a thing its documentation claims. Adding a
second dependency here would silently retire that check.

## No tests of its own, by design

The lane *is* the test: every claim it prints is an assertion. `tests/lane_test.rs`
exists anyway, because a lane that is its own test still needs *something to
run it* — no test suite can reach a bare `src/main.rs` to raise its coverage,
so the lane lives in `src/lib.rs` and the suite drives it.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — the facade as the single dependency |
| [`src/lib.rs`](src/lib.rs) | The lane: the exact path, the control arm, the asserted disagreement, the market split, and the ten scenes |
| [`src/main.rs`](src/main.rs) | The lane's process entry point, and nothing else |
| [`tests/lane_test.rs`](tests/lane_test.rs) | Runs the lane, each arm, the market split, and the scenes' golden values and checksum, from the suite |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Design documentation — the control-arm and library-not-bare-main decisions, the ledger pitfall, module index, workaround (none) |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_arith/`](../exact_arith/readme.md) — the only import
- `smoke_exact_arithmetic` — the predecessor lane this crate replaces at cutover; deleted from disk during this migration (recoverable via `git show`). Its `docs/decisions/` and `docs/pitfall/` are carried forward here, rewritten against this crate's own current source: [`docs/decisions/`](docs/decisions/readme.md), [`docs/pitfall/`](docs/pitfall/readme.md)
