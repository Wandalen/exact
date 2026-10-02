# exact_cmp

Tier 2 — comparison, equality, and min/max per kind, under the preferred
design's free-function names.

```rust
use exact_cmp::{ money_cmp, price_min };
use exact_kind::{ Money, Price };

let a = Money::parse( "1" ).unwrap();
let b = Money::parse( "2" ).unwrap();
assert_eq!( money_cmp( a, b ), core::cmp::Ordering::Less );
assert_eq!( price_min( Price::parse( "1" ).unwrap(), Price::parse( "2" ).unwrap() ).to_string(), "1" );
```

## Why unconditional, with no `CmpError`

The preferred design warns against an unconditional `Ord` where two values
being compared might silently carry different scales. That risk belongs to
a representation where scale is a runtime value; under this family's
const-generic `Decimal< SCALE >`, two different scales are two different
Rust types; a scale mismatch is already a compile error at the call site,
never a value these functions could receive. See the disclosed deviation in
[`src/lib.rs`](src/lib.rs)'s module doc comment.

## What it does not do

It defines no new comparison logic — every function here dispatches
directly to `exact_kind::Decimal`/`Qty`'s own derived `Ord`/`PartialEq`.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` alone |
| [`src/lib.rs`](src/lib.rs) | `money`/`qty`/`price_cmp`, `money_eq`, `price_min`/`max` |
| [`tests/cmp_test.rs`](tests/cmp_test.rs) | Dispatch correctness for ordering, equality, and extrema |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/workaround/`](docs/workaround/readme.md) | External constraints this crate absorbs — none |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the derived `Ord`/`PartialEq` this crate dispatches to
