# 001: Rounding

## Representation

How a value that falls between two representable grid points is placed onto
one of them. Seven variants: `Down` (floor), `Up` (ceiling), `HalfEven`
(nearest, ties to even — the only one with no directional bias over a long
run, which is why it is the family's default), `TowardZero` (truncation),
`AwayFromZero`, `HalfUp` (nearest, ties away from zero) and `HalfDown`
(nearest, ties toward zero). Net-new: no real
crate in the family's prior 5-crate shape offered more than one implicit
rounding behaviour, so this is written fresh against the preferred design's
own spec, not ported.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_round/src/lib.rs:38`

```rust
pub enum Rounding
{
  /// Round toward negative infinity — the floor.
  Down,

  /// Round toward positive infinity — the ceiling.
  Up,

  /// Round to the nearest grid point; on an exact tie, round to the point
  /// whose last digit is even.
  ///
  /// The tie-breaking rule a correctly-rounded decimal pipeline needs:
  /// rounding every tie the same direction biases a long sum of many
  /// rounded values, where biasing toward even cancels on average because
  /// ties land on an even last digit and an odd one equally often.
  HalfEven,

  /// Round toward zero — truncation: `2.7` to `2`, `-2.7` to `-2`.
  TowardZero,

  /// Round away from zero: `2.1` to `3`, `-2.1` to `-3`.
  AwayFromZero,

  /// Round to the nearest grid point; on an exact tie, round away from
  /// zero: `2.5` to `3`, `-2.5` to `-3`.
  HalfUp,

  /// Round to the nearest grid point; on an exact tie, round toward zero:
  /// `2.5` to `2`, `-2.5` to `-2`.
  HalfDown,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 38,78,80,88,90,92-98,136,158,190,192-198 | Return type of `rounding_default`; match subject/arms in `rounding_name` and `round_div_wide`; parameter of `round_div` and `round_div_wide` |
| `tests/rounding_mode_test.rs`, `tests/round_div_test.rs` | throughout | Every variant exercised directly, against hand-worked values and against each mode's definition on a grid |
| `exact_dust/src/lib.rs` (via `Rounding` parameter on every `money_dust_*`/`qty_dust_*` function) | — | **Production** — the rounding-mode parameter threaded through every dust-split function |
| `exact_snap/src/lib.rs` (via `rounding` parameter on `price_snap_tick`/`qty_snap_lot`) | — | **Production** |
| `exact_ratio/src/lib.rs` (via `rounding` parameter on `money_div_round`/`qty_div_round`, every `*_mul_ratio`, and `price_mul_qty`) | — | **Production** |
| `exact_arith/src/lib.rs:84` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:142` | — | **Production** — `Rounding::Down` passed to `money_dust_split` in the demo ledger |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised by its own tests |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the rounding-mode parameter every rounding-sensitive operation in these 3 crates takes |
| `exact_arith` | `src/lib.rs` | Re-export only |
| `smoke_exact_market_split` | `src/lib.rs` | **Production** — the demo ledger's dust-split call |
