# 023: Decimal::rescale

## Representation

The same value at another scale, `TO`. Going finer appends zeros — the stored
count is multiplied by `10^(TO - SCALE)`, exact under every mode, refused only
on overflow. Going coarser divides by `10^(SCALE - TO)` and rounds per the
caller's `Rounding`; `Rounding::Exact` refuses any digit that would be lost.
The result passes through `Decimal::< TO >::from_minor`, so the ceiling holds
at the new scale. The one explicit crossing between scales feature 019 asks
for (→ [ADR-002](../../decisions/002_explicit_rescale_between_scales.md)).

| Call | Stored before | Stored after |
|------|---------------|--------------|
| `12.34` at 2 → 6, any mode | `1234` | `12_340_000` |
| `12.345678` at 6 → 2, `HalfEven` | `12_345_678` | `1235` |
| `12.345678` at 6 → 2, `Exact` | `12_345_678` | `ExcessPrecision { digits : 6, scale : 2 }` |
| `1.005` at 6 → 2, `Exact` | `1_005_000` | `ExcessPrecision { digits : 3, scale : 2 }` — the value's own digits, not its scale |

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:261`

```rust
pub fn rescale< const TO : u32 >( self, rounding : Rounding ) -> Result< Decimal< TO >, KindError >
{
  let minor = self.minor();
  let rescaled = if TO >= SCALE
  {
    minor.checked_mul( pow10( TO - SCALE ) ).ok_or( KindError::Overflow { operation : "rescale" } )?
  }
  else
  {
    round_div( minor, pow10( SCALE - TO ), rounding ).map_err( | e | match e
    {
      RoundError::Inexact => KindError::ExcessPrecision { digits : fraction_digits( minor, SCALE ), scale : TO },
      RoundError::DivZero | RoundError::Overflow => KindError::Overflow { operation : "rescale" },
    } )?
  };
  Decimal::< TO >::from_minor( rescaled )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 261 | Declaration |
| `src/lib.rs` | 643,880 | `Qty::rescale` and `PriceAt::rescale` delegate to it |
| `tests/rescale_test.rs` | 12,20,21,23,31,32,48,49,59,60,61 | Finer is exact; coarser rounds per the mode at both signs; `Exact` refuses a dropped digit; past the ceiling is refused — `ExceedsCeiling` within the backing width (48), `Overflow` past it (49); `Exact` reports the value's own digits — `1.005` as 3, at both signs |
| `tests/price_at_test.rs` | 47,50,51,53 | Through `PriceAt::rescale`, including `1.005` refused as 3 digits |
| `smoke_exact_market_split/src/lib.rs:404` | — | Step 7 narrows the one-scale arm's cost back to cents |

Outside `exact_kind`, only the demo lane calls it — no production caller yet.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; `Qty`/`PriceAt` delegate to it; exercised by two test files |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: the one-scale arm of step 7 |

## Caller Tree

- **Intra-crate:** [Qty::rescale](024_rescale_qty.md) (`src/lib.rs:643`), and `PriceAt::rescale` (`src/lib.rs:880`, a member listed on [PriceAt](../struct/003_price_at.md))
- **External:** `smoke_exact_market_split::per_asset_trade` (`smoke_exact_market_split/src/lib.rs:404`) — demo-lane code, not production

## Callee Tree

- **Intra-crate:** [Decimal::minor](003_minor_decimal.md), [Decimal::from_minor](001_from_minor_decimal.md) at scale `TO`
- **External:** `exact_scale::pow10`; `exact_round::round_div` on the coarser branch (→ [use exact_round](../use_declaration/004_use_exact_round.md))
- `fraction_digits` (`src/lib.rs:990`, private — no Item Instance of its own), when `Exact` refuses: the value's own fractional digits, trailing zeros not counted
