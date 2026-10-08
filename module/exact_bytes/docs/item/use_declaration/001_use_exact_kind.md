# 001: use exact_kind::{ KindError, Money, Price, Quantity }

## Representation

Brings the three conserved-value types and their shared error type into
scope — the types for each `*_to_wire`/`*_from_wire` pair's signature,
`KindError` for `kind_error_to_wire_error`'s translation from an
`exact_kind` failure into this crate's own `WireError`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_bytes/src/lib.rs:39`

```rust
use exact_kind::{ KindError, Money, Price, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 39 | Declaration |
| `src/lib.rs` | 92,96-98 | `KindError` as `kind_error_to_wire_error`'s parameter type, and each of its five variants matched by name |
| `src/lib.rs` | 197,209,212,217,230,233,238,248,251 | `Money`/`Quantity`/`Price` across the 6 to/from-wire functions |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Parameter/return types and error-translation input for every public function |
