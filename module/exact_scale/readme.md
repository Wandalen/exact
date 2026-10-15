# exact_scale

Tier 0 — scale-factor math and the declared ceiling: the power-of-ten table,
the headroom factor, and the constants a range budget is checked against.

```rust
use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 };

assert_eq!( pow10( MONEY_SCALE ), 1_000_000 );
assert!( CEILING_MINOR_UNITS > 0 );
```

## Why a sibling root, not a dependent

This crate and [`exact_minor`](../exact_minor/readme.md) are both tier-0
roots of the family, with no edge between them — Tier 0 crates have no
dependencies on one another by design. So where the original `pow10()` this
crate is drawn from returned the family's `Backing` alias, this crate names
the backing primitive (`i64`) directly instead of importing across an edge
the dependency graph does not have. `exact_kind`, which depends on both
tier-0 roots, is where the two converge under one shared name again.

Scale stays a compile-time fact carried in a type's own const generic
parameter, not a runtime value — this crate holds the constants and the
power-of-ten table behind that mechanism, not a runtime `Scale` type. This is
a deliberate departure from the preferred design's own
[`docs/type/002_exact_scale_types.md`](../../docs/type/002_exact_scale_types.md),
which specifies a runtime `Scale(u8)` value; keeping the scale at
compile-time avoids turning this migration into an unrelated behavioural
rewrite of every same-scale check across the family.

## What it does not do

It declares no `ScaleError` — the family's one scale-level failure mode,
`pow10` exceeding the backing width, stays a panicking `const fn` matching
the real precedent it is drawn from, so there is no runtime scale error to
carry.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero workspace dependencies, by design as a tier-0 root |
| [`src/lib.rs`](src/lib.rs) | `pow10`, `HEADROOM_FACTOR`, `CEILING_WHOLE_UNITS`/`CEILING_MINOR_UNITS`, `MONEY_SCALE` |
| [`tests/scale_factor_test.rs`](tests/scale_factor_test.rs) | The power-of-ten table, the headroom relation, and the panic past the widest power |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Non-functional-requirement and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_minor/`](../exact_minor/readme.md) — the sibling tier-0 root naming the backing integer
- [`exact_round/`](../exact_round/readme.md) — the sibling tier-0 root naming the rounding modes
- [`exact_kind/`](../exact_kind/readme.md) — the tier-1 crate where this scale and the backing integer converge
