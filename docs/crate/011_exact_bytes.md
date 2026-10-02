# Crate: exact_bytes

### Scope

- **Purpose**: Fix `exact_bytes`'s dependency on `exact_kind` and `exact_scale`, and its role as the wire/save layer.
- **Responsibility**: Wire and save representation as minor plus scale.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function/error surface (→ `../type/011_exact_bytes_types.md`).

**Design status**: implemented in [`exact_bytes`](../../module/exact_bytes/readme.md). Dependencies match as specified (`exact_kind`, `exact_scale`); its error surface adds two variants beyond this proposal's three — see [`type/011_exact_bytes_types.md`](../type/011_exact_bytes_types.md) and [`exact_bytes`'s own decision](../../module/exact_bytes/docs/decisions/001_wire_error_overflow_and_negative_variants.md) for the full account.

### Why It Exists

Wire and save are minor plus scale, never a floating-point encoding.

### If Missing

JSON carries `f64`, reintroducing the family's foundational problem at the serialization boundary.

### Hard Problems Owned

10 (closed VM types), 14 (serialization).

### Features Owned

15 (bytes: minor plus scale, no float).

### Boundary

Not workstream 012's file format — this crate covers the value's own wire shape only.

### Dependencies

`exact_kind`, `exact_scale`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:334-342` | `exact_bytes`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:400` | Dependency-tree edge: `exact_bytes → exact_kind, exact_scale` |
