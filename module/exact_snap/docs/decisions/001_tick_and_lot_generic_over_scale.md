# ADR-001: A Tick And A Lot Carry Their Own Scale

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`price_snap_tick` and `qty_snap_lot` became generic over the scale with the
rest of the family (→ [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)),
but a grid's spacing was `Tick( Price )` and `Lot( Quantity )` — fixed at six
places. A price in cents could not be snapped to a tick in cents.

The exchange stores both as fields — `exchange_spec`'s `pub tick : Tick` and
`pub lot : Lot` — so the names must keep working without a scale.

## Decision

- `pub struct TickAt< const S : u32 >( PriceAt< S > )` and
  `pub struct LotAt< const S : u32 >( Qty< S > )`, with their members unchanged.
- `pub type Tick = TickAt< MONEY_SCALE >;` and `pub type Lot = LotAt< MONEY_SCALE >;`
  — the old names, at the old scale. Both are re-exported by `exact_arith`
  beside `TickAt`/`LotAt`.
- `exact_snap` gains a dependency on `exact_scale`, for `MONEY_SCALE`.

## Alternatives Considered

### Option 1: Make `Tick`/`Lot` themselves generic, with no alias

Rejected: `exchange_spec`'s `pub tick : Tick` field would need a scale, and
every other type-position use with it.

### Option 2: Re-export `MONEY_SCALE` from `exact_kind`

Rejected: it widens `exact_kind`'s public surface only to save one manifest
line. Depending on `exact_scale` directly is what `exact_kind` and
`exact_parse` already do.

### Option 3: Write the scale as a literal, `TickAt< 6 >`

Rejected: a second copy of the standard scale that could drift from
`MONEY_SCALE` with nothing to catch it.

## Consequences

**Positive:**
- A grid can be declared at the scale of the prices or quantities it snaps.
- `Tick::new( price )` and every use of `Tick`/`Lot` compile unchanged.

**Negative:**
- A new dependency edge, `exact_snap → exact_scale`. The crate doc
  ([`docs/crate/012_exact_snap.md`](../../../../docs/crate/012_exact_snap.md))
  listed `exact_kind` and `exact_round`; its Design status records the
  addition.

**Neutral:**
- A tick and the price it snaps must share one scale; a mismatch is a compile
  error.

## Related

- [`exact_arith`'s ADR-001](../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md) — the family-wide change this follows
- [`exact_kind`'s ADR-003](../../../exact_kind/docs/decisions/003_price_generic_over_scale.md) — the same alias pattern for `Price`
- [TickAt](../item/struct/001_tick_at.md), [LotAt](../item/struct/002_lot_at.md) — the items this changes
