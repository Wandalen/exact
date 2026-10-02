# Crate: exact_snap

### Scope

- **Purpose**: Fix `exact_snap`'s dependency on `exact_kind` and `exact_round`, and its role as the instrument-grid layer.
- **Responsibility**: Snapping price to a tick and quantity to a lot.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function/error surface (→ `../type/012_exact_snap_types.md`).

**Design status**: implemented in [`exact_snap`](../../module/exact_snap/readme.md). Dependencies match as specified (`exact_kind`, `exact_round`); its function surface diverges from this proposal — see [`type/012_exact_snap_types.md`](../type/012_exact_snap_types.md) for the account. A zero-sized tick or lot is refused at construction, but a negative grid spacing is deliberately accepted — `exact_round::round_div` already normalizes a negative divisor correctly, so there is no analogous failure to guard against — see [`exact_snap`'s own invariant doc](../../module/exact_snap/docs/invariant/001_grid_spacing_never_zero.md).

### Why It Exists

Snap price to tick and quantity to lot against an instrument's grid.

### If Missing

Workstream 002 reimplements tick/lot snapping itself, duplicating logic that belongs here.

### Hard Problems Owned

11 (tick and lot snap).

### Features Owned

17 (`snap_tick`, `snap_lot`).

### Boundary

No order book — that belongs to workstream 002.

### Dependencies

`exact_kind`, `exact_round`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:344-352` | `exact_snap`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:401` | Dependency-tree edge: `exact_snap → exact_kind, exact_round` |
