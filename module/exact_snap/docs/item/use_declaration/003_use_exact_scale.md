# 003: use exact_scale::MONEY_SCALE

## Representation

Brings in the standard scale, which fixes the `Tick` and `Lot` aliases
(→ [ADR-001](../../decisions/001_tick_and_lot_generic_over_scale.md)). The
crate's only use of `exact_scale` — everything else here is generic over `S`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_snap/src/lib.rs:26`

```rust
use exact_scale::MONEY_SCALE;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 26 | Declaration |
| `src/lib.rs` | 79,117 | `Tick = TickAt< MONEY_SCALE >`, `Lot = LotAt< MONEY_SCALE >` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | The scale the two aliases are fixed at |
