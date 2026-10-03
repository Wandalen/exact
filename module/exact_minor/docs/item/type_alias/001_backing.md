# 001: Backing

## Representation

The backing integer width for every conserved value in the family — `i64`.
Named exactly once, here, so a width change is one edit; every other crate in
the family re-exports this alias rather than restating `i64`. The crate's
single most-depended-upon item: both other consumers of `exact_minor`
(`exact_sign`, `exact_kind`) import it, while neither calls any of this
crate's functions (see each function's own Caller Tree).

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_minor/src/lib.rs:39`

```rust
pub type Backing = i64;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 39,51,92 | Return/parameter type of every function in the crate |
| `tests/checked_arithmetic_test.rs`, `tests/saturating_arithmetic_test.rs` | throughout | `Backing::MAX`/`MIN` boundary literals |
| `exact_sign/src/lib.rs:21,39,57,64,78` | — | **Production** — parameter type of every one of `exact_sign`'s 4 public functions |
| `exact_kind/src/lib.rs:52` (+34 more sites) | — | **Production** — the field type of `Decimal::minor` and the parameter/return type threaded through nearly every `Decimal`/`Qty` method (see `exact_kind`'s own `docs/item/use_declaration/001_use_exact_minor.md`) |
| `exact_kind/tests/checked_arithmetic_test.rs:15`, `tests/non_negative_test.rs:8` | — | `Backing::MAX`/`MIN` boundary literals |
| `exact_arith/src/lib.rs` | — | Re-exported as part of the facade's pure `pub use exact_minor::{ .. }` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | The width `Minor` wraps |
| `exact_sign` | `src/lib.rs` | **Production** — parameter type for sign classification, unrelated to this crate's own arithmetic functions |
| `exact_kind` | `src/lib.rs` | **Production** — `Decimal`'s storage field type and the type threaded through its entire API |
| `exact_arith` | `src/lib.rs` | Re-export only |
