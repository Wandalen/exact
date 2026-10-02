# 001: Rounding

## Representation

How a value that falls between two representable grid points is placed onto
one of them. Three variants: `Down` (floor), `Up` (ceiling), `HalfEven`
(nearest, ties to even — the only one of the three with no directional bias
over a long run, which is why it is the family's default). Net-new: no real
crate in the family's prior 5-crate shape offered more than one implicit
rounding behaviour, so this is written fresh against the preferred design's
own spec, not ported.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_round/src/lib.rs:27`

```rust
pub enum Rounding
{
  Down,
  Up,
  HalfEven,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 27,53,55,63,65,67,68,69,109,133,135,147,159 | Return type of `rounding_default`; match subject/arms in `rounding_name` and `round_div` |
| `tests/rounding_mode_test.rs`, `tests/round_div_test.rs` | throughout | Every variant exercised directly |
| `exact_dust/src/lib.rs` (via `Rounding` parameter on every `money_dust_*`/`qty_dust_*` function) | — | **Production** — the rounding-mode parameter threaded through every dust-split function |
| `exact_snap/src/lib.rs` (via `rounding` parameter on `price_snap_tick`/`qty_snap_lot`) | — | **Production** |
| `exact_ratio/src/lib.rs` (via `rounding` parameter on `money_div_round`/`qty_div_round`) | — | **Production** |
| `exact_arith/src/lib.rs:77` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:134` | — | **Production** — `Rounding::Down` passed to `money_dust_split` in the demo ledger |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised by its own tests |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the rounding-mode parameter every rounding-sensitive operation in these 3 crates takes |
| `exact_arith` | `src/lib.rs` | Re-export only |
| `smoke_exact_market_split` | `src/lib.rs` | **Production** — the demo ledger's dust-split call |
