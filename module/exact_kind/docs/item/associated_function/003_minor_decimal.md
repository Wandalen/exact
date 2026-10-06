# 003: Decimal::minor

## Representation

The count of minor units this value holds — the one escape hatch from the
type back to a raw integer.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:227`

```rust
pub const fn minor( self ) -> Backing
{
  minor_to_i64( self.minor )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 227,443,494 | Declaration; `Qty::from_decimal`'s negativity check; `Qty::minor`'s delegation |
| `tests/*.rs` (all 3) | throughout | Minor-count assertions |
| `exact_bytes/src/lib.rs:178,233` | — | `money_to_wire`/`price_to_wire` |
| `exact_conserve/src/lib.rs:250` | — | `money_sum_assert_zero`'s per-leg accumulation |
| `exact_dust/src/lib.rs:164,179,205` | — | `money_dust_split`/`_into`/`_remainder` |
| `exact_snap/src/lib.rs:81,141,143` | — | `Tick::new`'s zero check; `price_snap_tick` (price and tick) |
| `exact_ratio/src/lib.rs:163,188,208` | — | `money_mul_ratio`, `price_mul_ratio`, `money_div_round` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::from_decimal`'s refusal and `Qty::minor`'s delegation |
| `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the standard way every one of these 5 crates reads a `Money`/`Price` value's raw minor count |

## Caller Tree

- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:443`)
- [Qty::minor](014_minor_qty.md) (`src/lib.rs:494`)
- **External:** `exact_bytes::money_to_wire` (`:178`), `price_to_wire` (`:233`)
- **External:** `exact_conserve::money_sum_assert_zero` (`:250`)
- **External:** `exact_dust::money_dust_split` (`:164`), `money_dust_split_into` (`:179`), `money_dust_remainder` (`:205`)
- **External:** `exact_snap::Tick::new` (`:81`), `price_snap_tick` (`:141-142`)
- **External:** `exact_ratio::money_mul_ratio` (`:163`), `price_mul_ratio` (`:187`), `money_div_round` (`:208`)

## Callee Tree

- None — a pure field access.
