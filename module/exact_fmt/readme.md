# exact_fmt

Tier 2 — formatting per kind, and `fmt_into`, a buffer-writing primitive
that renders without allocating.

```rust
use exact_fmt::{ fmt_into, money_fmt };
use exact_kind::Money;

let v = Money::parse( "1.5" ).unwrap();
assert_eq!( money_fmt( v ), "1.5" );

let mut buf = [ 0_u8; 16 ];
let written = fmt_into( v, &mut buf ).unwrap();
assert_eq!( &buf[ .. written ], b"1.5" );
```

## Why `Display` stays in `exact_kind`

The preferred design calls for `Display` to be a thin wrapper over
`fmt_into`, implemented in this crate. That is impossible under Rust's
orphan rules given the dependency direction this migration chose: `exact_fmt`
depends on `exact_kind`, so this crate owns neither the `Display` trait nor
the `Decimal`/`Qty` types, and cannot implement a foreign trait for a
foreign type. See the disclosed deviation in
[`src/lib.rs`](src/lib.rs)'s module doc comment for the full reasoning.

## What it does not do

It implements no formatting logic of its own — every function here renders
through `exact_kind`'s existing `Display` impl, ported unchanged from the
real codebase.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_kind` alone |
| [`src/lib.rs`](src/lib.rs) | `money`/`qty`/`price_fmt`, `fmt_into`, `FmtError` |
| [`tests/fmt_test.rs`](tests/fmt_test.rs) | Per-kind rendering, the buffer primitive, and the too-small-buffer refusal |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Design documentation — rendering algorithm, module index, workaround (none) |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_kind/`](../exact_kind/readme.md) — the `Display` impl this crate renders through
