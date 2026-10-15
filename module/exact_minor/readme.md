# exact_minor

Tier 0 — the raw subunit integer and its checked and saturating arithmetic,
with no scale, no sign policy, and no kind attached.

```rust
use exact_minor::{ minor_checked_add, minor_from_i64, minor_to_i64 };

let sum = minor_checked_add( minor_from_i64( 300_000 ), minor_from_i64( 200_000 ) ).unwrap();
assert_eq!( minor_to_i64( sum ), 500_000 );
```

## Why a tier-0 root

Every other crate in the family eventually bottoms out at one backing
integer and one set of arithmetic primitives over it. Rather than let each
of those crates declare its own `i64` alias and re-derive checked add,
subtract, and negate, this crate names the width once and offers the
primitives as free functions over one small type, `Minor` — scale, sign,
and kind are each a separate, later tier's concern.

Saturating variants are new here; the family's prior shape only ever offered
checked operations. They exist for call sites that have already decided a
clamped answer to a range failure is acceptable, which checked arithmetic by
itself cannot express.

## What it does not do

It carries no scale and no non-negativity policy — both are a different
crate's responsibility ([`exact_scale`](../exact_scale/readme.md) and
[`exact_sign`](../exact_sign/readme.md) respectively). A `Minor` here is a
count of minor units with no decimal point and no sign rule attached to it.

## The types, as the design specifies them

- **`Minor`** wraps the backing `i64` (`Backing`). Use `minor_from_i64` /
  `minor_to_i64` to go in and out; a bare `i64` is refused by the compiler.
- **`MinorWide`** wraps an `i128`, only with `--features i128`. A `Minor`
  widens into it with `MinorWide::from`, and comes back with `Minor::try_from`;
  a raw `i128` goes in with `minor_wide_from_i128` and out with `minor_wide_to_i128`.
  It has the same arithmetic, prefixed `minor_wide_`.
- **`MinorError`** says which way an operation failed: `Overflow` (too big)
  or `Underflow` (too small).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero workspace dependencies, by design as a tier-0 root |
| [`src/lib.rs`](src/lib.rs) | `Backing`, checked and saturating minor-unit arithmetic, `MinorError` |
| [`tests/checked_arithmetic_test.rs`](tests/checked_arithmetic_test.rs) | In-range exactness and refusal at both backing extremes |
| [`tests/saturating_arithmetic_test.rs`](tests/saturating_arithmetic_test.rs) | Clamping behaviour at both backing extremes |
| [`tests/zero_test.rs`](tests/zero_test.rs) | `minor_zero` and `minor_is_zero` |
| [`tests/conversion_test.rs`](tests/conversion_test.rs) | `minor_from_i64`/`minor_to_i64` round trip and `Minor`'s ordering |
| [`tests/wide_test.rs`](tests/wide_test.rs) | `MinorWide` — widening and narrowing, zero, checked arithmetic in both directions, and clamping; compiled only with `--features i128` |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Invariant and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_scale/`](../exact_scale/readme.md) — the sibling tier-0 root naming the scale constants
- [`exact_round/`](../exact_round/readme.md) — the sibling tier-0 root naming the rounding modes
- [`exact_kind/`](../exact_kind/readme.md) — the tier-1 crate where this backing integer first gets a scale and a kind
