# 003: Price

## Representation

A price at the standard money scale — a struct wrapping one [`Money`](../type_alias/001_money.md),
so a price and an amount of money are different types and cannot be mixed
(hard problem 003, feature 003). Signed like `Money`: a price may be a
discount. Every member delegates to the wrapped `Money` and wraps the result
back, so no arithmetic is implemented a second time.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:640`

```rust
pub struct Price
{
  value : Money,
}
```

## Members

| Member | Line | Delegates to |
|--------|------|--------------|
| `ZERO` | `src/lib.rs:648` | `Money::ZERO` |
| `MAX` | `src/lib.rs:651` | `Money::MAX` |
| `from_minor` | `src/lib.rs:658` | [`Decimal::from_minor`](../associated_function/001_from_minor_decimal.md) |
| `minor` | `src/lib.rs:669` | [`Decimal::minor`](../associated_function/003_minor_decimal.md) |
| `checked_add` | `src/lib.rs:679` | [`Decimal::checked_add`](../associated_function/005_checked_add_decimal.md) |
| `checked_sub` | `src/lib.rs:693` | [`Decimal::checked_sub`](../associated_function/006_checked_sub_decimal.md) |
| `parse` | `src/lib.rs:707` | [`Decimal::parse`](../associated_function/009_parse_decimal.md) |
| `Display` | `src/lib.rs:714` | [`Display` for `Decimal`](../implementation/004_display_for_decimal.md) |

Only the members another crate uses exist; `Money`'s others (`from_int`,
`whole`, `checked_mul_int`, `checked_neg`, `EPSILON`, `MIN`) have no `Price`
caller.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 640-719 | Declaration, members and `Display` |
| `src/lib.rs` | 622-638 | `compile_fail` doctests: money plus a price, and a price plus money, do not compile |
| `tests/parse_render_test.rs` | 10,127-134 | A price may be negative and renders back exactly |
| `tests/price_test.rs` | throughout | Constants, range, exact arithmetic, parse/render and ordering of `Price` as its own kind |
| `exact_add/src/lib.rs:46,93,103` | — | `price_add`/`price_sub` |
| `exact_bytes/src/lib.rs:39,231,241,251` | — | `price_to_wire`/`price_from_wire` |
| `exact_cmp/src/lib.rs:32,50,64,71` | — | `price_cmp`/`price_min`/`price_max` |
| `exact_fmt/src/lib.rs:41,119` | — | `price_fmt` |
| `exact_parse/src/lib.rs:38,72,74` | — | `price_from_str` |
| `exact_ratio/src/lib.rs:46,185,188,235` | — | `price_mul_ratio`, `price_mul_qty` |
| `exact_snap/src/lib.rs:24,70,79,90,130,142` | — | `Tick`, `price_snap_tick` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; its own tests check the negative-price round trip and that money and price do not mix |
| `exact_add`, `exact_bytes`, `exact_cmp`, `exact_fmt`, `exact_parse`, `exact_ratio`, `exact_snap` | `src/lib.rs` | The `price_*` member of the `money_*`/`qty_*`/`price_*` set each of these crates exposes |
| `exact_arith` | `src/lib.rs` | Re-export only |
