# Crate: exact_conserve

### Scope

- **Purpose**: Fix `exact_conserve`'s dependency on `exact_add` and `exact_kind`, and its role as the audit layer.
- **Responsibility**: Verifying a slice of legs sums to zero.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/013_exact_conserve_types.md`).

**Design status**: implemented in [`exact_conserve`](../../module/exact_conserve/readme.md). Dependencies match as specified (`exact_add`, `exact_kind`) — this edge retires the predecessor `exact_audit`'s zero-dependency contract, recorded in [`exact_conserve`'s own decision](../../module/exact_conserve/docs/decisions/001_dependency_contract_retired_for_typed_layer.md); its function surface diverges — see [`type/013_exact_conserve_types.md`](../type/013_exact_conserve_types.md).

### Why It Exists

A slice of legs sums to zero — the family's conservation check.

### If Missing

No audit tool exists to catch a leak.

### Hard Problems Owned

5 (division and rounding — the remainder this crate audits).

### Features Owned

13 (`sum_assert_zero`), 14 (`ConservationError`).

### Boundary

No matching — matching logic belongs to workstream 002.

### Dependencies

`exact_add`, `exact_kind`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:354-362` | `exact_conserve`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:402` | Dependency-tree edge: `exact_conserve → exact_add, exact_kind` |
