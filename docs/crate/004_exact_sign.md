# Crate: exact_sign

### Scope

- **Purpose**: Fix `exact_sign`'s single dependency on `exact_minor` and its narrow responsibility for sign semantics.
- **Responsibility**: Zero/positive/negative classification, with no negative zero.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function/error surface (→ `../type/004_exact_sign_types.md`).

**Design status**: implemented in [`exact_sign`](../../module/exact_sign/readme.md). Dependencies and boundary match this proposal exactly — `exact_minor` alone, no division. The function surface deviates in one respect (a policy function replaces a bare per-kind flag) — see `../type/004_exact_sign_types.md` for the account.

### Why It Exists

Zero, positive, negative; no negative zero.

### If Missing

Escrow and debt collapse — sign edge cases have no defined behavior.

### Hard Problems Owned

9 (sign and zero).

### Features Owned

18 (`zero`, `is_zero`, `is_negative`, `sign`), 21 (neg only where the kind allows debt).

### Boundary

No division — that is `exact_ratio`'s concern.

### Dependencies

`exact_minor`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:264-272` | `exact_sign`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:393` | Dependency-tree edge: `exact_sign → exact_minor` |
