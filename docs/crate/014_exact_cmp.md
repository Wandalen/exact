# Crate: exact_cmp

### Scope

- **Purpose**: Fix `exact_cmp`'s single dependency on `exact_kind`, and its role as the exact-ordering layer.
- **Responsibility**: Comparison and ordering with no epsilon.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/014_exact_cmp_types.md`).

**Design status**: implemented in [`exact_cmp`](../../module/exact_cmp/readme.md). Dependencies match as specified (`exact_kind` alone); its function/error surface diverges — see [`type/014_exact_cmp_types.md`](../type/014_exact_cmp_types.md).

### Why It Exists

Exact order, no epsilon — comparisons the book can trust bit-for-bit.

### If Missing

Float compares creep into the book, reintroducing the family's foundational problem at the comparison boundary.

### Hard Problems Owned

7 (determinism).

### Features Owned

11 (exact `Eq` and `Ord`, no epsilon).

### Boundary

No levels — order-book level structure belongs to workstream 002.

### Dependencies

`exact_kind`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:364-372` | `exact_cmp`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:403` | Dependency-tree edge: `exact_cmp → exact_kind` |
