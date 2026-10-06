# 002: use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 }

## Representation

Brings in the declared ceiling, the standard money scale, and the power-of-
ten helper from the Tier-0 scale crate. `CEILING_MINOR_UNITS` bounds every
constructor's range check; `MONEY_SCALE` fixes `Money`/`Price`/`Quantity`'s
scale; `pow10` computes `Decimal::ONE_MINOR`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:54`

```rust
use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 54,57,640,60,125,176,189,192,201 | `MONEY_SCALE` fixes the three type aliases (56,62,65); `CEILING_MINOR_UNITS` bounds `from_minor`'s range check (186), renders in `KindError::Display` (130), and defines `Decimal::MAX`/`MIN` (174,177); `pow10` computes `ONE_MINOR` (161) |

No external file references this `use` declaration directly — other crates
depending on `exact_scale` import it independently.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Fixes the standard scale, the declared ceiling, and the power-of-ten helper used throughout `Decimal`/`Qty` |
