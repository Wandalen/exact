# Type: exact_ratio Types

### Scope

- **Purpose**: Specify `exact_ratio`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Ratio`, `mul_ratio`, and `div_round` per kind.
- **In Scope**: The struct, functions, and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/007_exact_ratio.md`).

**Design status**: implemented in [`exact_ratio`](../../module/exact_ratio/readme.md). The `Ratio { n, d }` struct and all six functions (`ratio_new`, `money`/`qty`/`price_mul_ratio`, `money`/`qty_div_round`) match this proposal's names exactly. `RatioError` diverges — it carries `DivZero`, `Overflow`, and `Negative { minor }`, dropping `ScaleMismatch` and `BadRounding` (both unreachable under this family's compile-time `SCALE` and closed `Rounding` enum) and adding `Negative` for a real failure case this proposal's listing missed — see [`exact_ratio`'s decision](../../module/exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) for the full account.

### Structs

- `Ratio { n: i64, d: i64 }`

### Functions

- `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio` — multiply by `n/d`.
- `money_div_round`, `qty_div_round` — mode-driven division.
- `ratio_new` — construction.

### Errors

- `RatioError { DivZero, Overflow, ScaleMismatch, BadRounding }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:463-468` | `exact_ratio`'s full struct/function/error listing |
