# 008: price_mul_qty_to

## Representation

The money a trade costs, `price × qty`, at the scale `M` the caller holds that
money in, rounded **once**. `price.minor × qty.minor` is the exact cost at
scale `P + Q`; it is divided by `10^(P + Q − M)` through
`exact_round::round_div_wide` with the caller's rounding, or — for an `M`
finer than `P + Q` — multiplied by `10^(M − P − Q)`, which is exact.

Converting [price_mul_qty](007_price_mul_qty.md)'s result with `rescale`
instead rounds twice, and can land a minor unit off: `0.14999999 × 0.1` is
exactly `0.014999999`, whose nearest cent is `0.01`, but rounding to the
price's 8 places first gives `0.015`, a tie that `HalfEven` takes to `0.02`
(review of #6, P6-1 → [ADR-003](../../decisions/003_cost_rounded_once_at_a_chosen_scale.md)).
`price_mul_qty` is this function with `M = P`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:289-305`

```rust
pub fn price_mul_qty_to< const M : u32, const P : u32, const Q : u32 >( price : PriceAt< P >, qty : Qty< Q >, rounding : Rounding ) -> Result< Decimal< M >, RatioError >
{
  // The exact cost at scale `P + Q`; both counts are within the ceiling, so the
  // `i128` product cannot overflow.
  let exact = i128::from( price.minor() ) * i128::from( qty.minor() );
  let minor = if M >= P + Q
  {
    exact.checked_mul( 10_i128.pow( M - ( P + Q ) ) ).ok_or( RatioError::Overflow )?
  }
  else
  {
    // The one rounding: straight from the exact cost to scale `M`.
    exact_round::round_div_wide( exact, 10_i128.pow( P + Q - M ), rounding ).map_err( round_error_to_ratio_error )?
  };
  let minor = i64::try_from( minor ).map_err( | _ | RatioError::Overflow )?;
  Decimal::< M >::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 289-305 | Declaration |
| `src/lib.rs` | 260 | `price_mul_qty`, which is this function with `M = P` |
| `src/lib.rs` | 272-282 | Doc example: `0.149 × 0.1` is `0.01` to the cent |
| `tests/price_mul_qty_to_test.rs` | throughout | The reviewer's double-rounding case; a negative price; equality with `price_mul_qty` at `M = P` under every mode; a finer `M` kept exact; `Exact` refusing; the ceiling checked at `M` |
| `exact_arith/src/lib.rs:110` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | `price_mul_qty` delegates to it; exercised by its own test file and doc example |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **Intra-crate:** [price_mul_qty](007_price_mul_qty.md) (`src/lib.rs:260`)

No external production caller yet; the exchange's settlement, holding cash at
its own scale, is the intended one.

## Callee Tree

- **External:** `exact_kind::PriceAt::minor`, `exact_kind::Qty::minor` (`src/lib.rs:293`) — the exact cost at scale `P + Q`, widened to `i128`
- **External:** `exact_round::round_div_wide` (`src/lib.rs:301`) — the one rounding, when `M` is coarser than `P + Q`
  - `round_error_to_ratio_error` (`src/lib.rs:119`, private — no Item Instance of its own) — `Inexact` under `Rounding::Exact`
- **External:** `exact_kind::Decimal::from_minor` at scale `M` (`src/lib.rs:304`) — the declared-ceiling check on the cost
  - `kind_error_to_ratio_error` (`src/lib.rs:84`, private — no Item Instance of its own)
