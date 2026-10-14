# 002: use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 }

## Representation

Brings in the declared ceiling, the standard money scale, and the power-of-
ten helper from the Tier-0 scale crate. `CEILING_MINOR_UNITS` bounds every
constructor's range check; `MONEY_SCALE` fixes `Money`/`Price`/`Quantity`'s
scale; `pow10` computes `Decimal::ONE_MINOR`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:56`

```rust
use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 59,62,65,130,276,294,297,307,362,366,505,554,559,583,585,1096,1097 | `MONEY_SCALE` fixes the three type aliases (59,62,65); `CEILING_MINOR_UNITS` renders in `KindError::Display` (130), defines `Decimal::MAX`/`MIN` (294,297) and bounds `from_minor`'s range check (307); `pow10` computes `ONE_MINOR` (276), `rescale`'s factor (362,366), `parse`'s padding (505), `render`'s rounding and digits (554,559,583,585) and `fraction_digits`'s divisors (1096,1097) |

No external file references this `use` declaration directly — other crates
depending on `exact_scale` import it independently.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Fixes the standard scale, the declared ceiling, and the power-of-ten helper used throughout `Decimal`/`Qty` |
