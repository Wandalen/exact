# exact_conserve

Tier 3 — conservation auditing: a plain-log verifier carried forward from
`exact_audit`, now netting each asset separately, plus a typed per-kind
convenience layer built on `exact_add`.

```rust
use exact_conserve::{ money_sum_assert_zero, Entry, verify };
use exact_kind::Money;

let log = [ Entry::new( "buyer", "cash", -1_000_000 ), Entry::new( "seller", "cash", 1_000_000 ) ];
assert!( verify( &log ).unwrap().is_balanced() );

let legs = [ Money::from_minor( 500 ).unwrap(), Money::from_minor( -500 ).unwrap() ];
assert_eq!( money_sum_assert_zero( &legs ), Ok( () ) );
```

## Why this crate is no longer zero-dependency

`exact_audit`'s own `docs/decisions/001_zero_dependency_by_contract.md` fixed
its manifest at no `[dependencies]` at all. The preferred design's own
dependency-tree edge (`exact_conserve → exact_add, exact_kind`) retires that
Contract here — the new typed layer genuinely needs both. `Entry`/`Report`/
`verify` themselves still touch neither crate. See the disclosed deviation in
[`src/lib.rs`](src/lib.rs)'s module doc comment.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exact_add` and `exact_kind` |
| [`src/lib.rs`](src/lib.rs) | `Entry`, `Report`, `verify`, `ConservationError`, `money`/`qty_conserve_into`, `money_sum_assert_zero` |
| [`tests/conservation_test.rs`](tests/conservation_test.rs) | Plain-log auditing (ported from `exact_audit`) plus the typed convenience layer |
| [`tests/manual/`](tests/manual/readme.md) | Manual-check plan and run record for this crate |
| [`docs/`](docs/readme.md) | Type, algorithm, decisions, and definition doc instances for this crate |
| [`verb/`](verb/readme.md) | Crate-scoped test/lint/build verb scripts |

## Related

- `exact_audit` — the crate `Entry`/`Report`/`verify` were carried forward from, before they gained a per-asset net; already removed from the tree as part of this migration, recoverable via `git show` against this repo's history
- [`exact_add/`](../exact_add/readme.md) — the checked arithmetic `money`/`qty_conserve_into` dispatch to
- [`exact_kind/`](../exact_kind/readme.md) — the conserved value types the typed layer operates on
