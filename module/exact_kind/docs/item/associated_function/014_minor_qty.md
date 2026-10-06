# 014: Qty::minor

## Representation

The count of minor units held, delegating to the wrapped decimal.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:492`

```rust
pub const fn minor( self ) -> Backing
{
  self.value.minor()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 492 | Declaration |
| `tests/non_negative_test.rs` | throughout | Minor-count assertions |
| `exact_bytes/src/lib.rs:205` | — | `qty_to_wire` |
| `exact_conserve/src/lib.rs:278` | — | `qty_sum_assert_zero`'s per-leg accumulation |
| `exact_dust/src/lib.rs:217,231,255` | — | `qty_dust_split`/`_into`/`_remainder` |
| `exact_snap/src/lib.rs:115,162,164` | — | `Lot::new`'s zero check; `qty_snap_lot` (qty and lot) |
| `exact_ratio/src/lib.rs:176,221,238,245` | — | `qty_mul_ratio`, `qty_div_round`; `price_mul_qty`'s quantity as a ratio, and the compile-time scale assert beside it |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own tests |
| `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the standard way every one of these 5 crates reads a `Quantity` value's raw minor count |

## Caller Tree

No intra-crate caller.

- **External:** `exact_bytes::qty_to_wire` (`:205`)
- **External:** `exact_conserve::qty_sum_assert_zero` (`:278`)
- **External:** `exact_dust::qty_dust_split` (`:217`), `qty_dust_split_into` (`:231`), `qty_dust_remainder` (`:255`)
- **External:** `exact_snap::Lot::new` (`:115`), `qty_snap_lot` (`:162,164`)
- **External:** `exact_ratio::qty_mul_ratio` (`:176`), `qty_div_round` (`:221`), `price_mul_qty` (`:238`), and the compile-time scale assert (`:245`)

## Callee Tree

- [Decimal::minor](003_minor_decimal.md) (`src/lib.rs:494`)
