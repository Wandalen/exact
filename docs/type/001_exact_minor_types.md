# Type: exact_minor Types

### Scope

- **Purpose**: Specify `exact_minor`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Minor`/`MinorWide` and their checked/saturating arithmetic.
- **In Scope**: Structs, functions, and the error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/001_exact_minor.md`).

**Design status**: implemented as specified in [`exact_minor`](../../module/exact_minor/readme.md) — `Minor(i64)` with `minor_from_i64`/`minor_to_i64`, `MinorWide(i128)` behind the `i128` feature, every checked and saturating function on `Minor`, and `MinorError { Overflow, Underflow }`. `MinorError`'s variants also carry the name of the failed operation. Beyond this listing, `MinorWide` has the same arithmetic as `Minor` (`minor_wide_*`) and converts back with `Minor::try_from`; like `Minor`, its field is private, with `minor_wide_from_i128`/`minor_wide_to_i128` as the raw-integer way in and out, mirroring `minor_from_i64`/`minor_to_i64`.

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
