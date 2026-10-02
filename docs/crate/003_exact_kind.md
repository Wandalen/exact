# Crate: exact_kind

### Scope

- **Purpose**: Fix `exact_kind`'s dependency on `exact_minor` and `exact_scale`, and its role as the newtype layer most other crates build on.
- **Responsibility**: The `Money`/`Qty`/`Price` distinct-newtype layer.
- **In Scope**: `Deps`, `Boundary`, hard problems and features this crate would own.
- **Out of Scope**: Its struct/function/error surface (→ `../type/003_exact_kind_types.md`).

**Design status**: implemented in [`exact_kind`](../../module/exact_kind/readme.md). Dependencies match this proposal exactly (`exact_minor`, `exact_scale`). The boundary deviates: checked arithmetic (`checked_add`, `checked_sub`, `checked_mul_int`, `checked_neg`) is implemented as inherent methods on `Decimal`/`Qty` here, not deferred entirely to `exact_add` — see [`exact_kind`'s own readme](../../module/exact_kind/readme.md)'s "What it does not do" section. The struct/function/error surface deviates further — see `../type/003_exact_kind_types.md` for the account.

### Why It Exists

Money, quantity, and price are distinct newtypes, not interchangeable numbers.

### If Missing

Credits add to kilograms — nothing in the type system stops a unit mix.

### Hard Problems Owned

3 (distinct kinds), 10 (closed VM types).

### Features Owned

3 (newtypes `Money`, `Qty`, `Price`).

### Boundary

No math — that is `exact_add`'s, `exact_ratio`'s, and siblings' concern.

### Dependencies

`exact_minor`, `exact_scale`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:254-262` | `exact_kind`'s Why/If-missing/Problems/Features/Boundary/Deps entry |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:394` | Dependency-tree edge: `exact_kind → exact_minor, exact_scale` |
