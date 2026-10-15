# Crate: exact_round

### Scope

- **Purpose**: Fix `exact_round`'s place at the root of the proposed dependency tree, owning the rounding-mode-as-data decision.
- **Responsibility**: The rounding mode type.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function surface (→ `../type/005_exact_round_types.md`).

**Design status**: implemented in [`exact_round`](../../module/exact_round/readme.md). Dependencies and boundary match this proposal — no dependencies, a tier-0 root alongside `exact_minor` and `exact_scale`, no decimal type. The crate's scope grew beyond this proposal's stated Features Owned: it also implements `round_div`/`RoundError`, a rounding division shared by `exact_ratio` and `exact_snap` — see [`round_div` Owned By `exact_round`](../../module/exact_round/docs/decisions/002_round_div_owned_by_exact_round.md). The struct/function surface otherwise deviates too — see `../type/005_exact_round_types.md` for the account.

### Why It Exists

The rounding mode is data, not an implicit per-caller choice.

### If Missing

Each caller truncates differently — no shared, named policy.

### Hard Problems Owned

5 (division and rounding).

### Features Owned

8 (rounding enum: down, up, half-even).

### Boundary

No decimal type — that is `exact_kind`'s concern.

### Dependencies

None — root of the dependency tree, alongside `exact_minor` and `exact_scale`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:274-282` | `exact_round`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:392` | Listed as a dependency-tree root |
