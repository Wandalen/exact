# 001: use exact_kind::{ Decimal, PriceAt, Qty }

## Representation

Brings the three conserved-value types into scope, in their generic form
(→ [`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)) — every function and the
one buffer-writing primitive in this crate renders one of them, always
through their own existing `Display` impl.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_fmt/src/lib.rs:41`

```rust
use exact_kind::{ Decimal, PriceAt, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 41 | Declaration |
| `src/lib.rs` | 105, 112, 119 | `Decimal< S >`/`Qty< S >`/`PriceAt< S >` parameter types on `money_fmt`/`qty_fmt`/`price_fmt` |

Does not import `KindError` — unlike `exact_add`/`exact_parse`/`exact_ratio`,
nothing in this crate is fallible at the `exact_kind` boundary; every
function here only reads an already-valid value through its `Display` impl,
which cannot fail.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Parameter types for all 3 per-kind render functions |
