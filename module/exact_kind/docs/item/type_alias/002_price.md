# 002: Price

## Representation

A price at the standard money scale — `PriceAt< MONEY_SCALE >`. The name every
caller outside generic code uses; mirrors [`Money`](001_money.md) and
[`Quantity`](003_quantity.md) for the price type. Before
[ADR-003](../../decisions/003_price_generic_over_scale.md) `Price` was the
struct itself, fixed at this scale; as an alias it keeps every member, the
scale and the text form it had.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:64`

```rust
pub type Price = PriceAt< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 64 | Declaration |
| `src/lib.rs` | 769-770,777-779,786-788 | Doctests on `PriceAt`: same-kind arithmetic, and money/price mixing refused |
| `tests/parse_render_test.rs` | 10,131 | A negative price round-trips |
| `tests/price_test.rs` | throughout | `Price` as its own kind at the standard scale |
| `tests/price_at_test.rs` | 4,11,37,47,49 | `Price` is `PriceAt< 6 >`; rescaling to and from it |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs:51,159,197,198,200` | — | The demo lane's tick size and snapped prices, through `exact_arith` |

No `price_*` function names `Price` in its signature: since
[`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)
each takes `PriceAt< S >`, and a `Price` reaches it as `S = MONEY_SCALE`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export |
| `smoke_exact_market_split` | `src/lib.rs` | Prices at the standard scale, through `exact_arith` |
