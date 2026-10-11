# ADR-003: A Price Carries Its Own Scale, As `PriceAt< SCALE >`

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`Decimal< SCALE >` and `Qty< SCALE >` were generic over their scale from the
start, but `Price` was a struct wrapping one `Money` — fixed at
`MONEY_SCALE`, six places. A price quoted in cents could be held only by
padding it to six places, and a price finer than six places — a token priced
at `0.00001234` — could not be held at all.

`Price` is used on 139 lines in this workspace and 67 in the exchange, and is
named throughout the docs, so whatever replaces it must leave plain `Price`
working.

## Decision

- `pub struct PriceAt< const SCALE : u32 > { value : Decimal< SCALE > }` — the
  same pattern as `Qty< SCALE >`. Every member (`ZERO`, `MAX`, `from_minor`,
  `minor`, `checked_add`, `checked_sub`, `parse`, `Display`) delegates to
  `Decimal< SCALE >` exactly as it delegated to `Money` before.
- `pub type Price = PriceAt< MONEY_SCALE >;` — the old name, at the old scale.
- `PriceAt::rescale` crosses scales (→ [ADR-002](002_explicit_rescale_between_scales.md)).
- A `compile_fail` doctest pins that `PriceAt< 2 >` plus `PriceAt< 6 >` does not
  compile.

## Alternatives Considered

### Option 1: `Price< const SCALE : u32 = MONEY_SCALE >`, a default const parameter

Rejected: a default fills the parameter in a type position only, not when
Rust infers a call. `let b = Price::from_minor( 7 );` fails with E0284, "type
annotations needed", so every untyped `Price::parse(…)` in the workspace would
break.

### Option 2: Make `Price` itself generic, with no alias

Rejected: every `Price` without a scale — 206 lines across the two
workspaces — becomes a compile error, for no gain over keeping the name as an
alias.

### Option 3: Keep `Price` at the money scale

Rejected: a price finer than six places cannot be represented, and an asset
quoted at another scale cannot share the family's operations without first
being rounded to six places.

## Consequences

**Positive:**
- A price can be held at whatever scale its market quotes in.
- Plain `Price` is unchanged: same scale, same members, same text form.

**Negative:**
- One more name to learn: `PriceAt< S >` for generic code, `Price` for the
  standard scale — the same split `Qty< S >`/`Quantity` already had.

**Neutral:**
- Prices at different scales stay different types, so mixing them is a compile
  error, as mixing a price with money already was.

## Related

- [ADR-002](002_explicit_rescale_between_scales.md) — `rescale`, which `PriceAt` delegates to
- [PriceAt](../item/struct/003_price_at.md) and [the `Price` alias](../item/type_alias/002_price.md) — the items this changes
- [Feature 003](../../../../docs/feature/003_newtypes_money_qty_price.md) — the requirement for a distinct price type; its Design status records this
- [Operations Are Generic Over The Scale](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md) — the family's functions that take a `PriceAt< S >`
