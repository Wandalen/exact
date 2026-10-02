# Crate: exact_minor

### Scope

- **Purpose**: Fix `exact_minor`'s place at the root of the proposed dependency tree — no crate below it, everything else eventually built on it.
- **Responsibility**: The raw integer subunit type and its checked arithmetic primitives.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function/error surface (→ `../type/001_exact_minor_types.md`).

**Design status**: implemented in [`exact_minor`](../../module/exact_minor/readme.md). This file's own scope — dependencies and boundary — matches the proposal exactly: no dependencies, a tier-0 root alongside `exact_scale` and `exact_round`, with no scale or kind concern. The code surface itself deviates (no `Minor` newtype, no `MinorWide`) — see `../type/001_exact_minor_types.md` for the account.

### Why It Exists

The raw integer subunit. Without it, every crate in the family would invent its own `i64` wrapper.

### If Missing

Every crate invents `i64` independently — no shared subunit type for the rest of the family to build on.

### Hard Problems Owned

2 (one canonical subunit), 4 (overflow).

### Features Owned

1 (minor as `i64`, optional `i128` feature).

### Boundary

No scale, no kind — those are `exact_scale`'s and `exact_kind`'s concerns.

### Dependencies

None — root of the dependency tree, alongside `exact_scale` and `exact_round`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:234-242` | `exact_minor`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:390` | Listed as a dependency-tree root |
