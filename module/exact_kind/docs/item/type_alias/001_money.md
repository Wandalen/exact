# 001: Money

## Representation

A value at the standard money scale — `Decimal< MONEY_SCALE >`. The
concrete, consumer-facing name every downstream crate (`exact_add`,
`exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_cmp`,
`exact_ratio`, `exact_dust`, `exact_conserve`) actually imports — the generic
`Decimal< const SCALE >` itself is rarely named outside this crate.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:56`

```rust
pub type Money = Decimal< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 56 | Declaration only — `exact_kind`'s own source never names `Money`, only generic `Decimal< SCALE >` |
| `tests/checked_arithmetic_test.rs`, `tests/parse_render_test.rs` | throughout | Concrete type exercised by this crate's own test suite |
| `exact_parse/src/lib.rs:36,43,45` | — | Compile-time `ONE_MINOR` assert; `money_from_str`'s parameter/return/body |
| `exact_bytes/src/lib.rs:163,175,185` | — | `money_to_wire`/`money_from_wire` signatures and bodies |
| `exact_conserve/src/lib.rs:223,245` | — | `money_conserve_into`/`money_sum_assert_zero` signatures |
| `exact_dust/src/lib.rs:151,156,166,172,184` | — | `money_dust_split`/`money_dust_split_into`/`money_dust_remainder` |
| `exact_add/src/lib.rs:35,45,95,110` | — | `money_add`/`money_sub`/`money_checked_neg`/`money_saturating_add` |
| `exact_fmt/src/lib.rs`, `exact_cmp/src/lib.rs` | — | Imported (`use exact_kind::{ Money, .. }`); re-exported `Display`/derived `Ord` exercised through it |
| `exact_ratio/src/lib.rs:140,143,186,189` | — | `money_mul_ratio`/`money_div_round` |
| `exact_arith/src/lib.rs:81` | — | Facade re-export (`pub use exact_kind::{ .., Money, .. }`) |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests |
| `exact_parse`, `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_add`, `exact_fmt`, `exact_cmp`, `exact_ratio` | `src/lib.rs` | The standard money type every one of these 8 Tier-2/3 crates' `money_*` functions operates on |
| `exact_arith` | `src/lib.rs` | Re-export only |
