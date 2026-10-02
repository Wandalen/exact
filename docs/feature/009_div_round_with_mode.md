# Feature: Div Round With Mode

### Scope

- **Purpose**: Divide two values under an explicitly named rounding mode instead of a language-default truncation.
- **Responsibility**: `exact_ratio`'s `_div_round` functions.
- **In Scope**: `money_div_round`, `qty_div_round`.
- **Out of Scope**: The rounding mode's own definition (→ Feature 008, `exact_round`).

**Design status**: implemented in [`exact_ratio`](../../module/exact_ratio/readme.md) as specified — `money_div_round`/`qty_div_round` both exist under these exact names, taking `mode` as a required argument. Two things beyond the function surface deviate: `RatioError` drops `ScaleMismatch`/`BadRounding` for `Negative` (same account as Feature 007 — see [`exact_ratio`'s own decision](../../module/exact_ratio/docs/decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md)), and the actual rounding-division arithmetic is implemented once in [`exact_round::round_div`](../../module/exact_round/docs/decisions/002_round_div_owned_by_exact_round.md) and called from here, rather than implemented inside this crate.

### Statement

`div_round(a, b, mode)` takes the rounding mode as a required argument, so a fee split's behavior is visible at the call site rather than buried in a crate-wide default.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:182` | Feature 9 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:463-468` | `exact_ratio`'s proposed `_div_round` functions |
