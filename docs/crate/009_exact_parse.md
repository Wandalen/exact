# Crate: exact_parse

### Scope

- **Purpose**: Fix `exact_parse`'s dependency on `exact_kind` and `exact_scale`, and its role as the string-to-minor layer.
- **Responsibility**: Turning a decimal string into minor units, or an error.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/009_exact_parse_types.md`).

**Design status**: implemented in [`exact_parse`](../../module/exact_parse/readme.md) as specified here — dependencies (`exact_kind`, `exact_scale`) match exactly, and the `"1.23"`-belongs-here feature is built and tested in this crate. Its function/error surface diverges in one respect (→ `../type/009_exact_parse_types.md`).

### Why It Exists

`"1.23"` becomes minor units, or an error — never a silent guess.

### If Missing

Callers reach for `parse::<f64>()`, reintroducing the family's foundational problem.

### Hard Problems Owned

1 (float money is wrong), 8 (display is not storage).

### Features Owned

5 (exact `from_str`, reject extra digits), 20 (reject non-finite and extra fractional digits at parse).

### Boundary

No display — that is `exact_fmt`'s concern.

### Dependencies

`exact_kind`, `exact_scale`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:314-322` | `exact_parse`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:398` | Dependency-tree edge: `exact_parse → exact_kind, exact_scale` |
