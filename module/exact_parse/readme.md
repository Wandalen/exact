# exact_parse

Tier 2 — text parsing per kind, under the preferred design's free-function
names. The `"1.23"` case the preferred design calls out as belonging here,
not in the eventual facade, is tested in this crate.

```rust
use exact_parse::money_from_str;

let v = money_from_str( "1.23" ).unwrap();
assert_eq!( v.minor(), 1_230_000 );
```

## Why a dispatch layer and not a reimplementation

The grammar — sign, integer digits, an optional fractional part of at most
`SCALE` digits — already lives in `exact_kind`'s parser, ported unchanged
from the real codebase's `exact_decimal::Decimal::parse`. This crate exists
to give that one grammar the preferred design's three per-kind names, not to
parse text a second time under a second implementation.

## What it does not do

It defines no `ParseError` and no standalone `parse_reject_extra_digits` —
see the disclosed deviations in [`src/lib.rs`](src/lib.rs)'s module doc
comment for why both are absent rather than merely unbuilt.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` and `exact_scale` |
| [`src/lib.rs`](src/lib.rs) | `money`/`qty`/`price_from_str`, and a cross-crate scale consistency guard |
| [`tests/from_str_test.rs`](tests/from_str_test.rs) | The `"1.23"` case per kind, the non-negativity refusal, and malformed text |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Design documentation — parsing algorithm, module index, workaround (none) |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the parser this crate dispatches to
- [`exact_scale/`](../exact_scale/readme.md) — the scale constant this crate's consistency guard checks against
