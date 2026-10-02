# exact_bytes

Tier 2 — `Wire`, a fixed-width byte encoding for a conserved value, and its
to/from conversions per kind. Net-new: the closest real precedent,
`exact_arithmetic`'s own `format/001_transaction_log_encoding.md`, was an
unimplemented spec for a bare 8-byte amount field.

```rust
use exact_bytes::{ money_to_wire, money_from_wire };
use exact_kind::Money;

let v = Money::parse( "1.5" ).unwrap();
let wire = money_to_wire( v );
let bytes = wire.to_bytes();
let decoded = exact_bytes::Wire::from_bytes( &bytes ).unwrap();
assert_eq!( money_from_wire( decoded ).unwrap(), v );
```

## Why self-describing rather than a bare amount

A bare 8-byte amount cannot be decoded back into a specific kind without an
external convention recording which kind and scale it was written at. This
crate's `Wire` widens that to 10 bytes — the minor-unit count, the scale,
and a kind discriminator — so a decoded record carries everything its own
`*_from_wire` function needs to validate it, rather than trusting the
reader's own bookkeeping to still agree with the writer's.

## What it does not do

It defines two error variants beyond the preferred design's own three —
`Overflow` and `Negative` — because `Wire`'s `minor` field is a bare `i64`
with no range check of its own; see the disclosed deviation in
[`src/lib.rs`](src/lib.rs)'s module doc comment.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` and `exact_scale` |
| [`src/lib.rs`](src/lib.rs) | `Wire`, `money`/`qty`/`price_to_wire`/`from_wire`, `WireError` |
| [`tests/wire_roundtrip_test.rs`](tests/wire_roundtrip_test.rs) | Round-tripping per kind, and every refusal: bad kind, bad scale, truncation, range |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/workaround/`](docs/workaround/readme.md) | External constraints this crate absorbs — none |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the conserved value types this crate encodes and decodes
- [`exact_scale/`](../exact_scale/readme.md) — the scale constant every wire record is checked against
