# Crate: exact_dust

### Scope

- **Purpose**: Fix `exact_dust`'s dependency on `exact_kind` and `exact_ratio`, and its role as the remainder-destination layer.
- **Responsibility**: Giving the last subunit of a split a destination.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its function/error surface (→ `../type/008_exact_dust_types.md`).

**Design status**: implemented in [`exact_dust`](../../module/exact_dust/readme.md). Diverges from this proposal — it depends on `exact_kind` and `exact_round`, not `exact_ratio`: the actual need is `exact_round`'s integer-count, mode-driven division (the same primitive `exact_snap` depends on directly), and none of `exact_ratio`'s rational-multiplier surface is used — see [`exact_dust`'s dependency decision](../../module/exact_dust/docs/decisions/001_direct_exact_round_dependency.md) for the full account. Its function/error surface also diverges (→ `../type/008_exact_dust_types.md`).

### Why It Exists

The last subunit of a split has a destination — fee sink or first party, never nowhere.

### If Missing

One unit vanishes on every split that doesn't divide evenly.

### Hard Problems Owned

6 (dust).

### Features Owned

10 (`remainder_assign` — where the dust goes).

### Boundary

No book — order-book concerns belong to workstream 002.

### Dependencies

`exact_kind`, `exact_ratio`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:304-312` | `exact_dust`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:397` | Dependency-tree edge: `exact_dust → exact_kind, exact_ratio` |
