# exact_add

Tier 2 — checked and saturating addition and subtraction, dispatched per
kind under the preferred design's free-function names.

```rust
use exact_add::{ money_add, money_saturating_add };
use exact_kind::Money;

let a = Money::parse( "0.1" ).unwrap();
let b = Money::parse( "0.2" ).unwrap();
assert_eq!( money_add( a, b ).unwrap(), Money::parse( "0.3" ).unwrap() );
assert_eq!( money_saturating_add( Money::MAX, Money::EPSILON ), Money::MAX );
```

## Why a dispatch layer and not a reimplementation

Every function here is a thin wrapper over a method `exact_kind` already
provides, named to match the preferred design's per-kind free-function
convention (`money_add`, `qty_add`, `price_add`) rather than the method-call
shape the real codebase used. Keeping the arithmetic itself in `exact_kind`
and only the naming convention here avoids two independent implementations
of the same checked addition under two different calling conventions.

## What it does not do

It defines no `AddError` and no panicking variant — see the disclosed
deviations in [`src/lib.rs`](src/lib.rs)'s module doc comment for why both
are absent rather than merely unbuilt.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` and `exact_sign` |
| [`src/lib.rs`](src/lib.rs) | `money`/`qty`/`price_add`/`sub`, `money_checked_neg`, saturating variants |
| [`tests/checked_and_saturating_add_test.rs`](tests/checked_and_saturating_add_test.rs) | Dispatch correctness and ceiling clamping at both signs |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Decisions and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the conserved value types this crate dispatches arithmetic to
- [`exact_sign/`](../exact_sign/readme.md) — the sign classification this crate's saturating clamp direction uses
