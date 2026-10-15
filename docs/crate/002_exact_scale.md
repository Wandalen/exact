# Crate: exact_scale

### Scope

- **Purpose**: Fix `exact_scale`'s place at the root of the proposed dependency tree, owning "digits after the point" as its own concern.
- **Responsibility**: The scale type and the same-scale check.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function/error surface (→ `../type/002_exact_scale_types.md`).

**Design status**: implemented in [`exact_scale`](../../module/exact_scale/readme.md). Dependencies and boundary match this proposal: no dependencies, a tier-0 root alongside `exact_minor` and `exact_round`, with no arithmetic of its own. The code surface deviates substantially — `Scale` was never built as a runtime type — see `../type/002_exact_scale_types.md` for the account.

### Why It Exists

Digits after the point, and answering "same scale?" as an explicit, checkable question.

### If Missing

Callers fall back to an implicit assumption of 2 digits, silently wrong for any other scale.

### Hard Problems Owned

2 (one canonical subunit), 13 (scale mismatch).

### Features Owned

2 (scale as `u8` digits after the point), 19 (`scale_convert`, explicit or error).

### Boundary

No arithmetic — that is `exact_add`'s, `exact_ratio`'s, and siblings' concern.

### Dependencies

None — root of the dependency tree, alongside `exact_minor` and `exact_round`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:244-252` | `exact_scale`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:391` | Listed as a dependency-tree root |
