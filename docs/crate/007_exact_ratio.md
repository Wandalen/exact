# Crate: exact_ratio

### Scope

- **Purpose**: Fix `exact_ratio`'s dependency on `exact_kind` and `exact_round`, and its role as the multiply/divide layer.
- **Responsibility**: `n/d` multiplication and mode-driven division.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/007_exact_ratio_types.md`).

**Design status**: implemented in [`exact_ratio`](../../module/exact_ratio/readme.md) as specified here — dependencies (`exact_kind`, `exact_round`) match exactly, and both owned features (`mul_ratio`, `div_round`) are built. Its function/error surface diverges in one respect (→ `../type/007_exact_ratio_types.md`).

### Why It Exists

Multiply by `n/d` and divide with an explicit mode.

### If Missing

Callers reach for float percentages, reintroducing the family's foundational problem.

### Hard Problems Owned

5 (division and rounding), 7 (determinism), 12 (hot path).

### Features Owned

7 (`mul_ratio(n, d)` checked), 9 (`div_round(a, b, mode)`).

### Boundary

No dust destination — that is `exact_dust`'s concern.

### Dependencies

`exact_kind`, `exact_round`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:294-302` | `exact_ratio`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:396` | Dependency-tree edge: `exact_ratio → exact_kind, exact_round` |
