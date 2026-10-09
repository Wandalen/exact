# 002: use exact_round::{ RoundError, Rounding }

## Representation

Brings in the rounding-mode enum every split takes, and the error type
`exact_round::round_div` can fail with — which this crate maps into its own
`DustError` via the private `round_error_to_dust_error` helper.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_dust/src/lib.rs:61`

```rust
use exact_round::{ RoundError, Rounding };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 61 | Declaration |
| `src/lib.rs` | 104 | `RoundError` match scrutinee in `round_error_to_dust_error` (private — no Item Instance of its own) |
| `src/lib.rs` | 122,159,176,207,218,231,243,253,263 | `Rounding` parameter type on `split_minor`, `split_with`, `split_into_with` (private) and all 6 public functions |

Deliberately depends on `exact_round` directly rather than through
`exact_ratio` — the module doc comment (`src/lib.rs:12-19`) discloses this:
the actual need is `exact_round::round_div` itself, which `exact_snap`
already establishes the direct-dependency precedent for; routing through
`exact_ratio` would add a dependency on its unused `Ratio`/`mul_ratio`
surface just to reach `round_div` indirectly.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | `Rounding` parameter on every public entry point; `RoundError` mapped internally |
