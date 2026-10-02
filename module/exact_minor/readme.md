# exact_minor

Tier 0 — the raw subunit integer and its checked and saturating arithmetic,
with no scale, no sign policy, and no kind attached.

```rust
use exact_minor::{ Backing, minor_checked_add };

let sum : Result< Backing, _ > = minor_checked_add( 300_000, 200_000 );
assert_eq!( sum, Ok( 500_000 ) );
```

## Why a tier-0 root

Every other crate in the family eventually bottoms out at one backing
integer and one set of arithmetic primitives over it. Rather than let each
of those crates declare its own `i64` alias and re-derive checked add,
subtract, and negate, this crate names the width once and offers the
primitives as free functions with no type wrapped around them yet — scale,
sign, and kind are each a separate, later tier's concern.

Saturating variants are new here; the family's prior shape only ever offered
checked operations. They exist for call sites that have already decided a
clamped answer to a range failure is acceptable, which checked arithmetic by
itself cannot express.

## What it does not do

It carries no scale and no non-negativity policy — both are a different
crate's responsibility ([`exact_scale`](../exact_scale/readme.md) and
[`exact_sign`](../exact_sign/readme.md) respectively, once built). A bare
`Backing` value here is a count of minor units with no decimal point and no
sign rule attached to it.

It also does not wrap that integer in a newtype. The preferred design's own
[`docs/type/001_exact_minor_types.md`](../../docs/type/001_exact_minor_types.md)
names a `Minor(i64)` struct; this crate instead exposes the backing width
directly as `pub type Backing = i64`, so there is nothing for the proposal's
`minor_from_i64`/`minor_to_i64` conversions to convert between, and neither
exists. The proposed `MinorWide(i128)` widening type, offered behind a
feature flag, was never built either — every arithmetic function here already
returns a `Result` on overflow rather than needing a wider intermediate type
to fall back to.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero workspace dependencies, by design as a tier-0 root |
| [`src/lib.rs`](src/lib.rs) | `Backing`, checked and saturating minor-unit arithmetic, `MinorError` |
| [`tests/checked_arithmetic_test.rs`](tests/checked_arithmetic_test.rs) | In-range exactness and refusal at both backing extremes |
| [`tests/saturating_arithmetic_test.rs`](tests/saturating_arithmetic_test.rs) | Clamping behaviour at both backing extremes |
| [`tests/zero_test.rs`](tests/zero_test.rs) | `minor_zero` and `minor_is_zero` |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Invariant and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_scale/`](../exact_scale/readme.md) — the sibling tier-0 root naming the scale constants
- [`exact_round/`](../exact_round/readme.md) — the sibling tier-0 root naming the rounding modes
- [`exact_kind/`](../exact_kind/readme.md) — the tier-1 crate where this backing integer first gets a scale and a kind
