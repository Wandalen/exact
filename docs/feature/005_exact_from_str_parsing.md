# Feature: Exact From Str Parsing

### Scope

- **Purpose**: Turn a decimal string into minor units without ever routing through a float.
- **Responsibility**: `exact_parse`'s `_from_str` functions.
- **In Scope**: `"1.23"` → minor units; rejecting extra digits beyond the declared scale.
- **Out of Scope**: Display, the inverse direction (→ `exact_fmt`).

**Design status**: implemented in [`exact_parse`](../../module/exact_parse/readme.md) as specified for the function surface — `money_from_str`, `qty_from_str`, `price_from_str` all exist under these exact names and never route through a float. Diverges on the error type: there is no `ParseError` — every function returns `exact_kind::KindError` directly, since each of the proposal's named `ParseError` variants either already exists under `KindError` or is unreachable under this family's compile-time scale — see [`exact_parse`'s own algorithm doc](../../module/exact_parse/docs/algorithm/001_decimal_parsing_per_kind.md)'s "Why No `ParseError`" section for the full account.

### Statement

Parsing goes directly from string to minor units, never through `str::parse::<f64>()`. A string carrying more fractional digits than the target scale allows is a parse error, not a silent truncation.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:174` | Feature 5 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:314-322` | `exact_parse`'s proposed responsibility and boundary |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:475-478` | `exact_parse`'s proposed functions and `ParseError` |
