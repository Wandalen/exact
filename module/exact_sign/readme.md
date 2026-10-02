# exact_sign

Tier 1 — sign classification over the family's backing integer, and the
negative-value admission policy `exact_kind` enforces non-negativity
through.

```rust
use exact_sign::{ Sign, sign_of, sign_neg_allowed };

assert_eq!( sign_of( -1 ), Sign::Neg );
assert!( !sign_neg_allowed( false, -1 ) );
```

## Why a policy function, not a per-kind constant

`sign_neg_allowed` takes the policy flag and the value together rather than
each kind hard-coding its own `if negative { Err } else { Ok }`. The call
reads at the use site as a question about the value under test. No crate
calls it yet: `exact_kind` enforces `Qty`'s non-negativity directly at
construction (`Qty::from_decimal`) rather than through this function, so it
is available policy, not yet the family's enforcement point.

## What it does not do

It holds no scale and no kind — it classifies a bare `Backing` value's sign
only. A `Qty`'s own refusal to hold a negative value is `exact_kind`'s
responsibility, built on top of this crate's policy function, not
duplicated here.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_minor` alone |
| [`src/lib.rs`](src/lib.rs) | `Sign`, `sign_of`, `is_negative`, `is_zero`, `sign_neg_allowed` |
| [`tests/sign_classification_test.rs`](tests/sign_classification_test.rs) | Classification at the zero boundary, and both admission policies |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Type, decisions, and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- [`exact_minor/`](../exact_minor/readme.md) — the backing integer this crate classifies the sign of
- [`exact_kind/`](../exact_kind/readme.md) — the tier-1 crate enforcing non-negativity through this crate's policy function
