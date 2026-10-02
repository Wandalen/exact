# Feature: Mul Ratio Checked

### Scope

- **Purpose**: Multiply a value by a `n/d` ratio without ever going through a float percentage.
- **Responsibility**: `exact_ratio`'s `_mul_ratio` functions.
- **In Scope**: `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio`.
- **Out of Scope**: Dust handling after the split (→ `exact_dust`).

**Design status**: implemented in [`exact_ratio`](../../module/exact_ratio/readme.md) as specified for the function surface — `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio` all take an explicit `Ratio` (built via `ratio_new`) and return a checked `Result`. `RatioError` deviates: it drops the proposal's `ScaleMismatch`/`BadRounding` (both unreachable under this family's compile-time scale and closed `Rounding` enum) and adds `Negative`, the one real failure — a `Qty` result going below zero — the proposal's listing missed. See [`exact_ratio`'s own decision](../../module/exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) for the full account.

### Statement

Fee and split math multiplies by an explicit `Ratio { n, d }` and returns a checked result, rather than computing a float percentage. This is the building block `exact_dust` and `exact_conserve` depend on to keep a split's parts exactly summing to the original.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:178` | Feature 7 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:294-302` | `exact_ratio`'s proposed responsibility and boundary |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:463-468` | `exact_ratio`'s proposed functions and `RatioError` |
