# exact_round

Tier 0 — rounding modes and the family's default policy: how a value that
falls between two representable grid points is placed onto one of them.

```rust
use exact_round::{ Rounding, rounding_default, round_div };

assert_eq!( rounding_default(), Rounding::HalfEven );
assert_eq!( round_div( 7, 2, Rounding::HalfEven ).unwrap(), 4 ); // 3.5 -> 4 (even)
```

## Why net-new, with no real-code precedent

The 5 real crates this family migration ports from never offered more than
one implicit rounding behaviour — there was nothing to extract. This crate
is written fresh against the preferred design's own crate specification.
`Rounding` started with zero consumers inside `module/` itself and
became load-bearing once [`exact_ratio`](../exact_ratio/readme.md)'s
`div_round` and [`exact_snap`](../exact_snap/readme.md)'s tick/lot snapping
were built in a later tier.

`HalfEven` is the chosen default, not `Down` or `Up`, because it is the only
one of the three with no directional bias over a long run of roundings — a
biased default would leak or manufacture value on every unrounded remainder,
silently, in one direction, forever, which is exactly what a conserved-value
family cannot afford.

## Why `round_div` lives here, not in its consumers

`exact_ratio`, `exact_snap` and `exact_dust` need "divide an integer by
another, applying a rounding mode to the remainder," and they already depend
on this crate for `Rounding` itself. Rather than let each hand-write its own
copy of the same sign-handling and tie-breaking logic, this crate owns it
once, where every consumer already has an edge — avoiding the duplication
without adding any of them a new dependency.

`round_div_wide` is that division over `i128` and holds the rounding rules
themselves; `round_div` widens its `i64` operands into it and narrows the
result back. `exact_ratio` calls `round_div_wide` directly for a ratio
multiply, whose product of two `i64` values no `i64` can hold.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero workspace dependencies, by design as a tier-0 root |
| [`src/lib.rs`](src/lib.rs) | `Rounding`, `rounding_default`, `rounding_name`, `round_div`, `round_div_wide`, `RoundError` |
| [`tests/rounding_mode_test.rs`](tests/rounding_mode_test.rs) | The default policy, stable names, and value semantics |
| [`tests/round_div_test.rs`](tests/round_div_test.rs) | `round_div` under every rounding mode, at both signs |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Type, algorithm, decisions, and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_minor/`](../exact_minor/readme.md) — the sibling tier-0 root naming the backing integer
- [`exact_scale/`](../exact_scale/readme.md) — the sibling tier-0 root naming the scale constants
- [`exact_ratio/`](../exact_ratio/readme.md) — the tier-2 crate dispatching `div_round` to this crate's `round_div`
- [`exact_snap/`](../exact_snap/readme.md) — the tier-2 crate dispatching tick/lot snapping to this crate's `round_div`
