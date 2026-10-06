# 002: use exact_round::Rounding

## Representation

Brings in the rounding-mode enum both snap functions take, so the caller
controls which way a value lands when it falls between two grid points.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_snap/src/lib.rs:25`

```rust
use exact_round::Rounding;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 25 | Declaration |
| `src/lib.rs` | 136,160 | `rounding` parameter type on `price_snap_tick`/`qty_snap_lot` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | Rounding-mode parameter type for both snap functions |
