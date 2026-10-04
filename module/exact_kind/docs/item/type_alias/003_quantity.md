# 003: Quantity

## Representation

A non-negative quantity at the standard money scale — `Qty< MONEY_SCALE >`.
The concrete name every downstream crate imports; mirrors
[`Money`](001_money.md) for the non-negative wrapper type instead of the
signed one.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:60`

```rust
pub type Quantity = Qty< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 60,407-408 | Declaration; struct-doc-comment doctest on `Qty` |
| `tests/non_negative_test.rs` | throughout | Concrete type exercised by this crate's own test suite |
| `exact_parse/src/lib.rs:53,55` | — | `qty_from_str` |
| `exact_bytes/src/lib.rs:190,192,203,213` | — | `qty_to_wire`/`qty_from_wire` |
| `exact_conserve/src/lib.rs:234,273` | — | `qty_conserve_into`/`qty_sum_assert_zero` |
| `exact_dust/src/lib.rs:196,201,210,216,226` | — | `qty_dust_split`/`qty_dust_split_into`/`qty_dust_remainder` |
| `exact_add/src/lib.rs:55,65,124` | — | `qty_add`/`qty_sub`/`qty_saturating_add` |
| `exact_fmt/src/lib.rs`, `exact_cmp/src/lib.rs` | — | Imported alongside `Money`/`Price` |
| `exact_snap/src/lib.rs:95,106,132` | — | `Lot::new`, `Lot::qty`, `qty_snap_lot` |
| `exact_ratio/src/lib.rs:153,156,198` | — | `qty_mul_ratio`/`qty_div_round` |
| `exact_arith/src/lib.rs:87` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests and its own struct-doc doctest |
| `exact_parse`, `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_add`, `exact_fmt`, `exact_cmp`, `exact_snap`, `exact_ratio` | `src/lib.rs` | The standard non-negative quantity type every one of these 9 crates' `qty_*` functions operates on |
| `exact_arith` | `src/lib.rs` | Re-export only |
