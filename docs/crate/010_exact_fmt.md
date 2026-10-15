# Crate: exact_fmt

### Scope

- **Purpose**: Fix `exact_fmt`'s single dependency on `exact_kind`, and its role as the display layer.
- **Responsibility**: Printing with scale while storage stays integer.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/010_exact_fmt_types.md`).

**Design status**: implemented in [`exact_fmt`](../../module/exact_fmt/readme.md) as specified here — its single dependency (`exact_kind`) matches exactly, and display-with-scale is built. Its function/error surface and `Display` placement diverge (→ `../type/010_exact_fmt_types.md`).

### Why It Exists

Print with scale; storage stays integer regardless of how it's displayed.

### If Missing

Callers print raw minors or floats — display and storage representation blur together.

### Hard Problems Owned

8 (display is not storage).

### Features Owned

16 (display with scale).

### Boundary

No parse — that is `exact_parse`'s concern.

### Dependencies

`exact_kind`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:324-332` | `exact_fmt`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:399` | Dependency-tree edge: `exact_fmt → exact_kind` |
