# 004: Decimal

## Representation

A fixed-point decimal carrying its scale in its type — money at any scale.
`SCALE` is the number of decimal places; the stored integer counts minor
units, so the denoted value is `minor × 10⁻ˢᶜᵃˡᵉ`. Ported from
`exact_decimal`'s `Decimal< const SCALE : u32 >` without behavioural change.
What it is built on comes from the sibling Tier-0 crates rather than being
declared again here: the stored count is an `exact_minor::Minor`, added,
subtracted and negated by `exact_minor`'s checked functions, and the scale
constants come from `exact_scale`.

An alias of [`Amount`](../struct/001_amount.md) of the money kind,
[`MoneyKind`](../struct/002_money_kind.md): `Decimal< SCALE >` is
`Amount< MoneyKind, SCALE >`. Until
[ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md) it was a
struct of its own, with the same one `minor` field. Its members are in
[its own `impl` block](../implementation/003_decimal_inherent_impl.md), on
`Amount< MoneyKind, SCALE >` alone, so `Qty` and `PriceAt` do not get them;
[`Money`](001_money.md) is the one at the standard scale.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:260`

```rust
pub type Decimal< const SCALE : u32 > = Amount< MoneyKind, SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 239-260 | Declaration, with its doctests: `0.1 + 0.2` is `0.3`, and a scale-6 value plus a scale-2 value does not compile (`compile_fail`) |
| `src/lib.rs` | 59 | Backs the `Money` alias |
| `src/lib.rs` | 214,222 | The private `value()` reads any kind as a `Decimal`; `from_value()` builds any kind from one |
| `src/lib.rs` | 273-515 | `impl Decimal` — every member, including `rescale`'s target `Decimal< TO >` at 357,372 |
| `src/lib.rs` | 517 | `Display` for `Decimal` |
| `src/lib.rs` | 658,661,664,671,688,702,715,798 | `Qty`'s constants, `from_decimal`, `from_minor`, `from_int`, `as_decimal` and `parse` go through a `Decimal` |
| `src/lib.rs` | 897,900,909,958 | `PriceAt`'s constants, `from_minor` and `parse` go through a `Decimal` |
| `src/lib.rs` | 987-989 | `MoneyKind`'s private hooks, `Decimal`'s own `checked_add`/`checked_sub` for `+` and `-` |
| `tests/*.rs` (8 files) | throughout | Direct `Decimal::< N >::parse`/`::from_minor` construction alongside `Money`, including `rescale_test.rs`, `operator_test.rs` and `amount_test.rs` |
| `exact_add/src/lib.rs:46,53,63,113,128,133` | — | `money_add`/`money_sub`/`money_checked_neg`/`money_saturating_add`, generic over the scale |
| `exact_bytes/src/lib.rs:39,199,211,214` | — | `money_to_wire`/`money_from_wire` |
| `exact_cmp/src/lib.rs:32,36,57` | — | `money_cmp`/`money_eq` |
| `exact_conserve/src/lib.rs:83,255,277` | — | `money_conserve_into`/`money_sum_assert_zero` |
| `exact_dust/src/lib.rs:62,210,212,221,223,234` | — | `money_dust_split`/`money_dust_split_into`/`money_dust_remainder` |
| `exact_fmt/src/lib.rs:41,105` | — | `money_fmt` |
| `exact_parse/src/lib.rs:39,53,55` | — | `money_from_str` |
| `exact_ratio/src/lib.rs:47,177,180,221,224,258,289,304` | — | `money_mul_ratio`/`money_div_round`; `price_mul_qty`'s cost, and `price_mul_qty_to`'s at the scale `M` the caller picks |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:51,153,155,183,184,185,187,190,207,351,372,375` | — | The demo lane's scale-2 scenes — the sum, the dust split, the wire round trip — and step 7's cost in cents, through `exact_arith` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The money kind of `Amount`, which `Money` aliases at the standard scale |
| `exact_add`, `exact_bytes`, `exact_cmp`, `exact_conserve`, `exact_dust`, `exact_fmt`, `exact_parse`, `exact_ratio` | `src/lib.rs` | The money type every `money_*` function takes, as `Decimal< S >` (→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)) |
| `exact_arith` | `src/lib.rs` | Re-export |
| `smoke_exact_market_split` | `src/lib.rs` | Amounts at scale 2, through `exact_arith` |
