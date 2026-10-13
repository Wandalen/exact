# 001: use exact_kind::{ Decimal, KindError, PriceAt, Qty }

## Representation

Brings the three conserved-value types, in their generic form, and their
shared error type into scope — the types for each `*_to_wire`/`*_from_wire`
pair's signature, at any scale `S`
(→ [ADR-002](../../decisions/002_scale_byte_from_the_type.md)), and
`KindError` for `kind_error_to_wire_error`'s translation from an `exact_kind`
failure into this crate's own `WireError`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_bytes/src/lib.rs:39`

```rust
use exact_kind::{ Decimal, KindError, PriceAt, Qty };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 39 | Declaration |
| `src/lib.rs` | 94,98-100 | `KindError` as `kind_error_to_wire_error`'s parameter type, and each of its five variants matched by name |
| `src/lib.rs` | 199,211,214,219,232,235,240,250,253 | `Decimal`/`Qty`/`PriceAt` across the 6 to/from-wire functions |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Parameter/return types and error-translation input for every public function |
