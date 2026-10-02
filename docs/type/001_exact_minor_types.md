# Type: exact_minor Types

### Scope

- **Purpose**: Specify `exact_minor`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Minor`/`MinorWide` and their checked/saturating arithmetic.
- **In Scope**: Structs, functions, and the error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/001_exact_minor.md`).

**Design status**: implemented in [`exact_minor`](../../module/exact_minor/readme.md), diverging in representation but not in arithmetic surface. `Minor(i64)` was not built as a newtype — the real crate exposes the backing width directly as `pub type Backing = i64` (see its [`docs/readme.md`](../../module/exact_minor/docs/readme.md)), so the `minor_from_i64`/`minor_to_i64` conversions this proposal named have nothing to convert between and don't exist; `MinorWide`/`i128` was never built. Every arithmetic function proposed here (`minor_zero`, `minor_is_zero`, `minor_checked_add`, `minor_checked_sub`, `minor_checked_neg`, `minor_saturating_add`, `minor_saturating_sub`) exists under the same name. `MinorError` carries one `Overflow { operation }` variant rather than a separate `Overflow`/`Underflow` split — see [Checked Operations Total](../../module/exact_minor/docs/invariant/002_checked_operations_total.md) for why the split would have no observable behaviour behind it.

### Structs

- `Minor(i64)` — the base subunit type.
- `MinorWide(i128)` — behind a feature flag, not a separate crate.

### Functions

- `minor_from_i64`, `minor_to_i64` — conversion.
- `minor_checked_add`, `minor_checked_sub`, `minor_checked_neg` — checked arithmetic.
- `minor_saturating_add`, `minor_saturating_sub` — saturating arithmetic.
- `minor_zero`, `minor_is_zero` — zero handling.

### Errors

- `MinorError { Overflow, Underflow }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:426-433` | `exact_minor`'s full struct/function/error listing |
