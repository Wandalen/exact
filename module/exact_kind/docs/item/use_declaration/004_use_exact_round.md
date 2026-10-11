# 004: use exact_round::{ RoundError, Rounding, round_div }

## Representation

Brings in the family's rounding mode, its rounded division and that division's
error, for [Decimal::rescale](../associated_function/023_rescale_decimal.md)'s
coarser branch — the same `round_div` `exact_ratio`, `exact_snap` and
`exact_dust` share, rather than a third copy here
(→ [ADR-002](../../decisions/002_explicit_rescale_between_scales.md)).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:54`

```rust
use exact_round::{ RoundError, Rounding, round_div };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 54 | Declaration |
| `src/lib.rs` | 246,536,772 | `Rounding` — the `rounding` parameter of `Decimal`/`Qty`/`PriceAt::rescale` |
| `src/lib.rs` | 255-258 | `round_div` divides on the coarser branch; `RoundError`'s three variants map to `KindError` |

No external file references this `use` declaration directly — other crates
depending on `exact_round` import it independently.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The rounding `rescale` takes and drives |
