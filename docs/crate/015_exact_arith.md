# Crate: exact_arith

### Scope

- **Purpose**: Fix `exact_arith`'s dependency on all 14 leaf crates, and its role as the sole facade workstream 002 would need to compile against.
- **Responsibility**: Re-exporting the leaves; no new policy.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its re-export surface (→ `../type/015_exact_arith_types.md`); the old, differently-scoped facade `exact_arithmetic` this proposal was once contrasted against — deleted at the migration's cutover and no longer on disk, retrievable only via `git show HEAD:exact_arithmetic/<path>`.

**Design status**: implemented in [`exact_arith`](../../module/exact_arith/readme.md), which replaced the differently-scoped `exact_arithmetic` at the 15-crate migration's cutover (the old crate no longer exists on disk; retrievable via `git show HEAD:exact_arithmetic/readme.md`). Diverges from this proposal's Dependencies — the real facade depends directly on, and re-exports, all 14 leaves (including the 3 named here as roots "reached transitively," plus `exact_sign`, which this proposal's list omits entirely) rather than 10 direct dependencies with 3 reached only transitively — see the disclosed deviation in [`exact_arith`'s own module doc](../../module/exact_arith/src/lib.rs).

### Why It Exists

Facade the others call, so workstream 002 does not wire fourteen crates individually.

### If Missing

Workstream 002 wires fourteen crates itself instead of one.

### Hard Problems Owned

None unique — composition only.

### Features Owned

4 (`from_minor`/`to_minor` re-export), plus composition of the other 14 crates' surfaces.

### Boundary

No policy beyond the leaves — this crate adds no behavior the leaves don't already have.

### Dependencies

`exact_kind`, `exact_add`, `exact_ratio`, `exact_dust`, `exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_conserve`, `exact_cmp` (all 10 non-root leaves; the 3 roots `exact_minor`/`exact_scale`/`exact_round` are reached transitively).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:374-384` | `exact_arith`'s Why/If-missing/Problems/Features/Boundary/Deps entry, plus the note that `i128` is a feature flag on `exact_minor`, not a separate crate |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:404` | Dependency-tree edge: `exact_arith → kind, add, ratio, dust, parse, fmt, bytes, snap, conserve, cmp` |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:408` | "002 depends on exact_arith (or directly on exact_kind, exact_cmp, exact_snap, exact_conserve if you want a narrower edge)" |
