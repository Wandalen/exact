# 006: PriceAt

## Representation

A price at scale `SCALE`, so a price and an amount of money are different
types and cannot be mixed (hard problem 003, feature 003), and two prices at
different scales are different types too. Signed like `Money`: a price may
be a discount. Every member delegates to the same count read as a
`Decimal` and builds the result back, so no arithmetic is implemented a
second time. [`Price`](002_price.md) is the one at the standard money scale
(→ [ADR-003](../../decisions/003_price_generic_over_scale.md)).

An alias of [`Amount`](../struct/001_amount.md) of the price kind,
[`PriceKind`](../struct/004_price_kind.md): `PriceAt< SCALE >` is
`Amount< PriceKind, SCALE >`. Until
[ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md) it was a
struct of its own, holding one `Decimal< SCALE >` as a `value` field; its
members now read it with the private `value()` and build with the private
`from_value()`.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:892`

```rust
pub type PriceAt< const SCALE : u32 > = Amount< PriceKind, SCALE >;
```

## Members

| Member | Line | Delegates to |
|--------|------|--------------|
| `ZERO` | `src/lib.rs:897` | [`Decimal::ZERO`](../associated_constant/002_zero_decimal.md) |
| `MAX` | `src/lib.rs:900` | [`Decimal::MAX`](../associated_constant/004_max_decimal.md) |
| `from_minor` | `src/lib.rs:907` | [`Decimal::from_minor`](../associated_function/001_from_minor_decimal.md) |
| `minor` | `src/lib.rs:918` | [`Decimal::minor`](../associated_function/003_minor_decimal.md) |
| `checked_add` | `src/lib.rs:928` | [`Decimal::checked_add`](../associated_function/005_checked_add_decimal.md); `PriceKind`'s hook (`src/lib.rs:1002`) hands it to [`+`](../implementation/008_add_for_amount.md) |
| `checked_sub` | `src/lib.rs:942` | [`Decimal::checked_sub`](../associated_function/006_checked_sub_decimal.md); `PriceKind`'s hook (`src/lib.rs:1003`) hands it to [`-`](../implementation/010_sub_for_amount.md) |
| `parse` | `src/lib.rs:956` | [`Decimal::parse`](../associated_function/009_parse_decimal.md) |
| `rescale` | `src/lib.rs:966` | [`Decimal::rescale`](../associated_function/023_rescale_decimal.md) |
| `Display` | `src/lib.rs:973` | [`Display` for `Decimal`](../implementation/004_display_for_decimal.md) |

Only the members another crate uses exist, plus `rescale`; `Decimal`'s others
(`from_int`, `whole`, `checked_mul_int`, `checked_neg`, `EPSILON`, `MIN`) have
no `PriceAt` caller. They are not inherited: `Decimal`'s `impl` block is on
`Amount< MoneyKind, SCALE >` alone.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 853-892 | Declaration, with its doctests (below) |
| `src/lib.rs` | 894-970 | `impl PriceAt` — the members above |
| `src/lib.rs` | 972-980 | `Display` for `PriceAt` |
| `src/lib.rs` | 65 | The `Price` alias, `PriceAt< MONEY_SCALE >` |
| `src/lib.rs` | 860-864, 868-891 | A runnable doctest — two prices add — and `compile_fail` doctests: money plus a price, a price plus money, and two prices at different scales do not compile |
| `src/lib.rs` | 1001-1003 | `PriceKind`'s private hooks, `PriceAt`'s own `checked_add`/`checked_sub` for `+` and `-` |
| `src/lib.rs` | 1033-1036 | `+`'s `compile_fail` example: a `Price` plus a `Quantity` |
| `tests/operator_test.rs` | 43-44,61-65 | `+`, `-` and chains on `PriceAt< 2 >` |
| `tests/amount_test.rs` | 17,39 | One `i64` at scale 18; `Debug` prints `PriceAt` |
| `tests/parse_render_test.rs` | 10,131 | A price may be negative and renders back exactly |
| `tests/price_test.rs` | throughout | Constants, range, exact arithmetic, parse/render and ordering of `Price` as its own kind |
| `tests/price_at_test.rs` | throughout | A price in cents: parse, add, render, refuse a third place; the ceiling at every scale; `rescale` |
| `exact_add/src/lib.rs:46,93,103` | — | `price_add`/`price_sub` |
| `exact_bytes/src/lib.rs:39,240,250,253` | — | `price_to_wire`/`price_from_wire` |
| `exact_cmp/src/lib.rs:32,50,64,71` | — | `price_cmp`/`price_min`/`price_max` |
| `exact_fmt/src/lib.rs:41,119` | — | `price_fmt` |
| `exact_parse/src/lib.rs:39,73,75` | — | `price_from_str` |
| `exact_ratio/src/lib.rs:47,203,206,258` | — | `price_mul_ratio`, `price_mul_qty` |
| `exact_snap/src/lib.rs:24,76,89,96,105,150,165` | — | `TickAt`, `TickAt::new`'s stored magnitude, `price_snap_tick` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs` | throughout | Prices in cents through the facade: add, compare, render, parse, snap, `price_mul_qty` |
| `smoke_exact_market_split/src/lib.rs:51,368,392` | — | Step 7's price in cents, `PriceAt< 2 >`, kept on `PerAsset` and printed with `{:.2}`, through `exact_arith` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; its own tests check the negative-price round trip, that money and price do not mix, and a price at scale 2 |
| `exact_add`, `exact_bytes`, `exact_cmp`, `exact_fmt`, `exact_parse`, `exact_ratio`, `exact_snap` | `src/lib.rs` | The `price_*` member of the `money_*`/`qty_*`/`price_*` set each of these crates exposes, generic over the scale |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export, and the any-scale facade test |
| `smoke_exact_market_split` | `src/lib.rs` | A price in cents in the demo lane's step 7 |
