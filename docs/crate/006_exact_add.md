# Crate: exact_add

### Scope

- **Purpose**: Fix `exact_add`'s dependency on `exact_kind` and `exact_sign`, and its role as the checked-arithmetic layer.
- **Responsibility**: Checked and saturating add/sub over the three kinds.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/006_exact_add_types.md`).

**Design status**: implemented in [`exact_add`](../../module/exact_add/readme.md). Dependencies match exactly (`exact_kind`, `exact_sign`). Checked and saturating add/sub are built as specified; the panicking variant named among this crate's owned features was deferred as YAGNI — no consumer calls for it yet — see [`exact_add`'s decision](../../module/exact_add/docs/decisions/002_no_panicking_variant_yet.md).

### Why It Exists

Checked and saturating add and sub, named explicitly rather than left to operator overloading.

### If Missing

Silent wrap — an overflow becomes a wrong value instead of a caught error.

### Hard Problems Owned

4 (overflow), 12 (hot path).

### Features Owned

6 (checked add and sub), 12 (checked/saturating/panicking variants, named).

### Boundary

No ratio — that is `exact_ratio`'s concern.

### Dependencies

`exact_kind`, `exact_sign`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:284-292` | `exact_add`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:395` | Dependency-tree edge: `exact_add → exact_kind, exact_sign` |
