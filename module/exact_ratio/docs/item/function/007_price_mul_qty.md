# 007: price_mul_qty

## Representation

The money a trade costs: `price × qty`, rounded per `rounding`, at the
price's own scale — [price_mul_qty_to](008_price_mul_qty_to.md) with `M = P`,
to which it delegates. A fractional quantity counts in full. It replaces the pattern
`price.checked_mul_int( qty.whole() )`, which silently drops the quantity's
fractional part (`1.25 × 4.5` came out as `5`, not `5.625`). The price and
the quantity may sit at different scales — a price in cents, a quantity in
satoshis — and one whole quantity is counted at the quantity's own scale; the
cost is money at the price's scale
(→ [ADR-002](../../decisions/002_price_mul_qty_across_two_scales.md)). Under
`Rounding::Exact` a cost that does not land on the price's scale is refused as
`RatioError::Inexact` — a settlement amount that is never rounded.

To hold the cost at another scale, call `price_mul_qty_to` rather than
`rescale` this result: rescaling rounds a second time, which can land a minor
unit off (→ [ADR-003](../../decisions/003_cost_rounded_once_at_a_chosen_scale.md)).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:258-261`

```rust
pub fn price_mul_qty< const P : u32, const Q : u32 >( price : PriceAt< P >, qty : Qty< Q >, rounding : Rounding ) -> Result< Decimal< P >, RatioError >
{
  price_mul_qty_to::< P, P, Q >( price, qty, rounding )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 258-261 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 183,192,204,325,411,414,416,418 | A fractional quantity; under `Exact`, the exchange's own `notional` cases — exact costs kept, a cost finer than one minor unit `Inexact`, one past the ceiling `Overflow`; a cost finer than one minor unit under `Down`/`Up`/`HalfEven`; a cost past the ceiling; a negative price and a zero quantity |
| `exact_arith/src/lib.rs:109` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:25` | — | The settlement test's notional, through the facade |
| `exact_arith/tests/any_scale_test.rs:31,33` | — | A price in cents times a quantity in satoshis, under `HalfEven` and `Exact` |
| `smoke_exact_market_split/src/lib.rs:392,398` | — | Step 7: cents × satoshis, and the same trade at one shared scale |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs`, `tests/any_scale_test.rs` | Re-export; its settlement test computes the trade's notional with it, and the any-scale test prices cents times satoshis |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: step 7 prices one trade at two scales |

## Caller Tree

- **External:** `exact_arith`'s own test, `facade_test.rs:25` (test-context, via the re-exported name)
- **External:** `smoke_exact_market_split::per_asset_trade` (`smoke_exact_market_split/src/lib.rs:392,398`) — demo-lane code

No production caller inside this repository yet.

## Callee Tree

- [price_mul_qty_to](008_price_mul_qty_to.md) (`src/lib.rs:260`), with the cost's scale `M` set to the price's `P`
