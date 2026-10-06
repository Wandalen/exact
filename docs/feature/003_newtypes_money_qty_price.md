# Feature: Newtypes Money Qty Price

### Scope

- **Purpose**: Give money, quantity, and price distinct types, so they cannot be added to each other by accident.
- **Responsibility**: `exact_kind`'s three newtypes.
- **In Scope**: `Money`, `Qty`, `Price` as separate structs over the same minor/scale pair.
- **Out of Scope**: The arithmetic operating on them (→ `exact_add`, `exact_ratio`).

**Design status**: implemented in [`exact_kind`](../../module/exact_kind/readme.md), with one real deviation: the shared shape is one generic `Decimal<const SCALE: u32>` carrying scale as a compile-time parameter rather than a runtime field each struct holds separately. On top of it, `Money` is `Decimal<MONEY_SCALE>`, `Quantity` wraps `Decimal` and refuses negative values, and `Price` wraps a `Money` — three distinct newtypes, so mixing any two is a compile error, pinned by `compile_fail` doctests. See [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) and [its non-negativity decision](../../module/exact_kind/docs/decisions/001_non_negativity_enforced_at_construction.md) for the full account.

### Statement

`Money`, `Qty`, and `Price` are three distinct newtypes wrapping the same `{ minor, scale }` shape. Without this separation, credits could add to kilograms with no compiler error — the newtype boundary is what makes that a type error instead of a runtime bug.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:170` | Feature 3 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:68-74` | Hard problem 3, "Distinct kinds," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:440-448` | `exact_kind`'s proposed struct definitions |
