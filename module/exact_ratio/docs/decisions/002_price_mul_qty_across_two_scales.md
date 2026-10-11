# ADR-002: `price_mul_qty` Multiplies Across Two Scales

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`price_mul_qty` treats the quantity as a ratio — its minor-unit count over one
whole unit — and counted one whole unit as `Money::ONE_MINOR`. That holds only
while the quantity and the money share one scale, so a compile-time assertion
beside the function failed the build if they ever drifted apart.

Once every operation is generic over the scale
(→ [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)),
a price and a quantity have independent scales — a price in cents, a quantity
in satoshis — and "one whole unit" has to be the quantity's own.

## Decision

- `price_mul_qty< const P : u32, const Q : u32 >( price : PriceAt< P >, qty : Qty< Q >, rounding ) -> Result< Decimal< P >, RatioError >`.
- One whole quantity is `Decimal::< Q >::ONE_MINOR` — the quantity's own scale.
- The cost is money at the price's scale `P`: a price is money per unit, so a
  price times a count of units is money at the price's precision.
- The compile-time assertion that `Quantity` and `Money` share one scale is
  removed: nothing assumes it any more.

$65 000.00 (`P = 2`) × 0.12345678 BTC (`Q = 8`):

| Step | Value |
|------|-------|
| `price.minor()` | `6_500_000` |
| `qty.minor()` over `Decimal::< 8 >::ONE_MINOR` | `12_345_678 / 100_000_000` |
| Product, in cents | `802_469.07` |
| `Rounding::HalfEven` | `802_469` → `8024.69` |
| `Rounding::Exact` | `RatioError::Inexact` |

## Alternatives Considered

### Option 1: One scale for both, `price_mul_qty< const S : u32 >`

Rejected: the motivating case is a price and a quantity at different scales.
One shared `S` would force the quantity to be rescaled to the price's scale
first, losing the digits this change exists to keep.

### Option 2: A third parameter for the cost's scale

Rejected under YAGNI: the cost's scale would appear only in the return type,
so every call would need a turbofish. A caller who needs the cost at another
scale can `rescale` it (→ [`exact_kind`'s ADR-002](../../../exact_kind/docs/decisions/002_explicit_rescale_between_scales.md)).

### Option 3: The cost at the quantity's scale

Rejected: the cost is money, whose precision is the price's, not the
quantity's.

## Consequences

**Positive:**
- A trade is priced exactly when price and quantity are quoted at different
  precisions.
- At the standard scale (`P = Q = 6`) the result is unchanged.

**Negative:**
- None identified for existing callers: both parameters are inferred from the
  arguments.

**Neutral:**
- The removed assertion was the only place the family required quantity and
  money to share a scale.

## Related

- [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md) — the family-wide change
- [price_mul_qty](../item/function/007_price_mul_qty.md) — the item this changes
