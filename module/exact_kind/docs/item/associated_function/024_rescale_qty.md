# 024: Qty::rescale

## Representation

The same quantity at another scale, `TO`, exactly as
[Decimal::rescale](023_rescale_decimal.md) computes it, then back through
`Qty::from_decimal`. Every rounding mode rounds a non-negative value to a
non-negative one, so the non-negativity check can never fail here — it is kept
so the result is built through the type's one gate, as every other `Qty`
constructor is.

`0.12345678` at 8 → 6 is `123_456` minor units under `Down`, and
`ExcessPrecision { digits : 8, scale : 6 }` under `Exact`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:536`

```rust
pub fn rescale< const TO : u32 >( self, rounding : Rounding ) -> Result< Qty< TO >, KindError >
{
  Qty::from_decimal( self.value.rescale::< TO >( rounding )? )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 536 | Declaration |
| `tests/rescale_test.rs` | 40,41 | A quantity in satoshis rescaled to six places, under `Down` and `Exact` |
| `smoke_exact_market_split/src/lib.rs:397` | — | Step 7's one-scale arm rounds a satoshi quantity to six places |

Outside `exact_kind`, only the demo lane calls it — no production caller yet.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own test |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: the one-scale arm of step 7 |

## Caller Tree

- **External:** `smoke_exact_market_split::per_asset_trade` (`smoke_exact_market_split/src/lib.rs:397`) — demo-lane code, not production

No production caller.

## Callee Tree

- **Intra-crate:** [Decimal::rescale](023_rescale_decimal.md), [Qty::from_decimal](010_from_decimal_qty.md)
