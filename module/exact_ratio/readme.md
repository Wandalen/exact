# exact_ratio

Tier 2 — a rational multiplier (`Ratio`) and mode-driven integer division,
per kind. Net-new: no real precedent exists for either operation.

```rust
use exact_kind::Money;
use exact_ratio::{ ratio_new, money_mul_ratio, money_div_round };
use exact_round::Rounding;

let half = ratio_new( 1, 2 ).unwrap();
let v = Money::parse( "10" ).unwrap();
assert_eq!( money_mul_ratio( v, half, Rounding::HalfEven ).unwrap(), Money::parse( "5" ).unwrap() );
assert_eq!( money_div_round( v, 4, Rounding::HalfEven ).unwrap(), Money::parse( "2.5" ).unwrap() );
```

## Why the multiply widens before it divides

A rate stated as `n / d` must multiply by `n` before dividing by `d` to stay
exact — dividing first rounds twice. But a value already near the declared
ceiling times a free-standing `n` can overflow `i64` long before the result
or either operand would. This crate widens every product to `i128` before
narrowing the quotient back down, per this family's own documented range
budget, rather than risk the narrower width on an operation it was
explicitly written to warn about.

## What it does not do

It defines no `ScaleMismatch` or `BadRounding` variant — see the disclosed
deviations in [`src/lib.rs`](src/lib.rs)'s module doc comment for why both
are unreachable rather than merely unbuilt.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` and `exact_round` |
| [`src/lib.rs`](src/lib.rs) | `Ratio`, `ratio_new`, `*_mul_ratio`, `*_div_round`, `RatioError` |
| [`tests/ratio_and_div_round_test.rs`](tests/ratio_and_div_round_test.rs) | Normalization, the widened multiply, and every rounding mode at both signs |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Design documentation — the `Ratio` type, the widened-multiply algorithm, error-shape decisions, module index, workaround (none) |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the conserved value types this crate multiplies and divides
- [`exact_round/`](../exact_round/readme.md) — the rounding modes `div_round` dispatches on
