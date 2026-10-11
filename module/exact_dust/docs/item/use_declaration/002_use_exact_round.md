# 002: use exact_round::{ RoundError, Rounding }

## Representation

Brings in the rounding-mode enum every split takes, and the error type
`exact_round::round_div` can fail with — which this crate maps into its own
`DustError` via the private `round_error_to_dust_error` helper.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_dust/src/lib.rs:63`

```rust
use exact_round::{ RoundError, Rounding };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 63 | Declaration |
| `src/lib.rs` | 107 | `RoundError` match scrutinee in `round_error_to_dust_error` (private — no Item Instance of its own) |
| `src/lib.rs` | 125,162,179,210,221,234,246,256,266,275,308 | `Rounding` parameter type on `split_minor`, `split_with`, `split_into_with` (private) and all 6 public functions; `weighted_share` and `money_dust_split_weighted` |

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
