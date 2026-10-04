# exact_kind

Tier 1 — the conserved value type family: `Money`, `Qty`, and `Price`, each a
fixed-point decimal whose scale lives in the type. Consolidates what the real
codebase built as two crates, `exact_decimal` and `exact_qty`, into the one
crate the preferred design names `exact_kind`.

```rust
use exact_kind::{ Quantity, Money };

let held = Quantity::from_int( 3 ).unwrap();
let taken = Quantity::from_int( 5 ).unwrap();
assert!( held.checked_sub( taken ).is_err() );

let a = Money::parse( "0.1" ).unwrap();
let b = Money::parse( "0.2" ).unwrap();
assert_eq!( a.checked_add( b ).unwrap(), Money::parse( "0.3" ).unwrap() );
```

## Why one crate for three kinds

`Money`, `Qty`, and `Price` all bottom out at the same fixed-point
representation over the same backing integer and the same scale constants —
splitting them into separate crates would either duplicate that
representation three times or force an awkward dependency between
otherwise-sibling kinds. One crate owning the whole family keeps there being
exactly one implementation of scale and exactly one declaration of the
backing width, same as the real codebase's own `exact_decimal` already
established.

`Qty` stays a genuinely separate type, not an alias: its refusal to hold a
negative value is real, tested, behaviourally-distinguishing logic, carried
forward unchanged from the real `exact_qty`. `Price` is a separate type too —
a struct wrapping a `Money` — so a price and an amount of money cannot be
mixed.

## What it does not do

It carries no arithmetic beyond its own methods — `exact_add` provides the preferred design's per-kind free-function names (`money_add`,
`qty_add`, `price_add`) as a thin dispatch layer over the methods here,
rather than this crate reimplementing arithmetic twice under two different
calling conventions.

It declares no `Scaled` trait and no `KindError::ScaleMismatch` — see the
disclosed deviations in [`src/lib.rs`](src/lib.rs)'s module doc comment for
why both are absent rather than merely unbuilt.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_minor` and `exact_scale` |
| [`src/lib.rs`](src/lib.rs) | `Decimal`, `Qty`, `Price`, the `Money`/`Quantity` aliases, `KindError` |
| [`tests/checked_arithmetic_test.rs`](tests/checked_arithmetic_test.rs) | Test Matrix T02-T04 — exactness and refusal at the edge, ported from `exact_decimal` |
| [`tests/parse_render_test.rs`](tests/parse_render_test.rs) | Test Matrix T01 — round-tripping through text, ported from `exact_decimal` |
| [`tests/non_negative_test.rs`](tests/non_negative_test.rs) | Test Matrix T05-T06 — the refusal, its exact boundary, and integer round-tripping, ported from `exact_qty` |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Invariant, type, decisions, algorithm, and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_minor/`](../exact_minor/readme.md) — the backing integer this crate's `Decimal` is built over
- [`exact_scale/`](../exact_scale/readme.md) — the scale constants and declared ceiling this crate inherits
- [`exact_sign/`](../exact_sign/readme.md) — the sibling tier-1 crate whose policy function this crate's non-negativity refusal is built against
- [`exact_add/`](../exact_add/readme.md) — the tier-2 crate dispatching arithmetic to this crate's methods under the preferred design's per-kind names
