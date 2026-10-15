# ADR-003: A Trade's Cost Is Rounded Once, At The Scale The Caller Holds

**Date**: 2026-10-12
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

[ADR-002](002_price_mul_qty_across_two_scales.md) made `price_mul_qty` take a
price and a quantity at two scales, landing the cost at the price's scale `P`.
It rejected a parameter for the cost's scale (its Option 2) on the grounds that
"a caller who needs the cost at another scale can `rescale` it".

Review of #6 (P6-1) showed the two are not equivalent: rescaling is a second
rounding, and two roundings can land on a different minor unit than one.

| Step | Value |
|------|-------|
| `0.14999999` (scale 8) × `0.1` (scale 1), exactly | `0.014999999` — the nearest cent is `0.01` |
| `price_mul_qty`, `HalfEven`, to scale 8 | `0.01500000` — now exactly half a cent |
| `.rescale::< 2 >( HalfEven )` | `0.02` — a tie, taken to the even cent |
| one `HalfEven` rounding to cents | `0.01` |

It happens whenever the price has more places than the currency is held in —
a token priced to 8 places against cash in cents, a normal setup once each
asset has its own scale.

## Decision

- A new function, `price_mul_qty_to< const M : u32, const P : u32, const Q : u32 >( price : PriceAt< P >, qty : Qty< Q >, rounding ) -> Result< Decimal< M >, RatioError >`:
  - `price.minor × qty.minor`, in `i128`, is the exact cost at scale `P + Q`.
  - For `M` coarser than `P + Q` it is divided by `10^(P + Q − M)` through
    `exact_round::round_div_wide` — the one rounding, under the caller's mode.
  - For `M` at least `P + Q` it is multiplied by `10^(M − P − Q)`, checked —
    exact, no rounding.
  - The result passes through `Decimal::< M >::from_minor`, so the ceiling holds
    at `M`, and a scale past 18 does not compile.
- `price_mul_qty` stays, unchanged in signature and result: it is
  `price_mul_qty_to::< P, P, Q >`, and now delegates to it, so there is one code
  path. Its documentation points callers who hold the cost at another scale to
  `price_mul_qty_to`.
- This supersedes ADR-002's Option 2 rejection. ADR-002's other decisions —
  independent price and quantity scales, the cost at the price's scale for
  `price_mul_qty` — stand.

## Alternatives Considered

### Option 1: Keep "the caller can rescale"

Rejected: a cent off on a settlement is the defect this family exists to
prevent.

### Option 2: Change `price_mul_qty` itself to take `M`

Rejected: `M` appears only in the return type, so every existing call would need
a scale named — the breaking change [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)
already pays for six return-only functions. A second, named function keeps
`price_mul_qty` unchanged.

### Option 3: Compute through a `Ratio`, as before

`price_mul_qty` built `qty.minor / 10^Q` as a `Ratio` and multiplied through
`mul_ratio_minor`. Rejected for the new function: the divisor `10^(P + Q − M)`
can reach `10^36`, past the `i64` a `Ratio` holds. One `i128` product and one
`i128` division hold every case.

## Consequences

**Positive:**
- A cost lands on the minor unit its rounding mode promises, at whatever scale
  the money is held in.
- `Rounding::Exact` refuses exactly the costs that do not fit scale `M`.

**Negative:**
- A caller that names the cost's scale writes it, either as a typed binding
  (`let cents : Decimal< 2 > = price_mul_qty_to( … )`) or as
  `price_mul_qty_to::< 2, _, _ >( … )`.

**Neutral:**
- `ratio_new` loses its one production caller inside this crate; it remains the
  public constructor for `Ratio`.

## Related

- [ADR-002](002_price_mul_qty_across_two_scales.md) — the two-scale cost this refines
- [price_mul_qty_to](../item/function/008_price_mul_qty_to.md) — the new function
- [price_mul_qty](../item/function/007_price_mul_qty.md) — now its `M = P` case
