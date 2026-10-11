# 001: Decimal

## Representation

A fixed-point decimal carrying its scale in its type. `SCALE` is the number
of decimal places; the stored integer counts minor units, so the denoted
value is `minor × 10⁻ˢᶜᵃˡᵉ`. Ported from `exact_decimal`'s `Decimal< const
SCALE : u32 >` without behavioural change. What it is built on now comes
from the sibling Tier-0 crates rather than being declared again here: the
stored count is an `exact_minor::Minor`, added, subtracted and negated by
`exact_minor`'s checked functions, and the scale constants come from
`exact_scale`.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:161`

```rust
pub struct Decimal< const SCALE : u32 >
{
  minor : Minor,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 58,143-150,161,177,193,196,203,219,231,246,261,267,279,283,293,297,309,316,327,331,347,402,406,502,504,523,525,527,540,554,567,569,747,753,756,765,814 | Backs the `Money` alias and `PriceAt`'s `value` field; receiver/return type of every `Decimal` method, including `rescale`'s target; backs `Qty`'s `value` field |
| `tests/*.rs` (5 files) | throughout | Direct `Decimal::< N >::parse`/`::from_minor` construction alongside `Money`, including `rescale_test.rs` and `price_at_test.rs` |
| `exact_add/src/lib.rs:46,53,63,113,128,133` | — | `money_add`/`money_sub`/`money_checked_neg`/`money_saturating_add`, generic over the scale |
| `exact_bytes/src/lib.rs:39,199,211,214` | — | `money_to_wire`/`money_from_wire` |
| `exact_cmp/src/lib.rs:32,36,57` | — | `money_cmp`/`money_eq` |
| `exact_conserve/src/lib.rs:83,255,277` | — | `money_conserve_into`/`money_sum_assert_zero` |
| `exact_dust/src/lib.rs:62,210,212,221,223,234` | — | `money_dust_split`/`money_dust_split_into`/`money_dust_remainder` |
| `exact_fmt/src/lib.rs:41,105` | — | `money_fmt` |
| `exact_parse/src/lib.rs:39,53,55` | — | `money_from_str` |
| `exact_ratio/src/lib.rs:47,177,180,221,224,256,259,261` | — | `money_mul_ratio`/`money_div_round`; `price_mul_qty`'s cost and its quantity's `ONE_MINOR` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:51,153,155,183,184,185,187,190,207,351,372,375` | — | The demo lane's scale-2 scenes — the sum, the dust split, the wire round trip — and step 7's cost in cents, through `exact_arith` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The signed fixed-point representation `Money` aliases and `Qty` and `PriceAt` wrap |
| `exact_add`, `exact_bytes`, `exact_cmp`, `exact_conserve`, `exact_dust`, `exact_fmt`, `exact_parse`, `exact_ratio` | `src/lib.rs` | The money type every `money_*` function takes, as `Decimal< S >` (→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)) |
| `exact_arith` | `src/lib.rs` | Re-export |
| `smoke_exact_market_split` | `src/lib.rs` | Amounts at scale 2, through `exact_arith` |
