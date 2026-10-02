# Hard Problem: Distinct Kinds

### Scope

- **Purpose**: State why money, quantity, and price must be different types.
- **Responsibility**: The requirement that the three kinds never unify into one bare number.
- **In Scope**: `Money`, `Qty`, `Price`.
- **Out of Scope**: The newtypes themselves (→ `../feature/003_newtypes_money_qty_price.md`).

**Design status**: implemented in [`exact_kind`](../../module/exact_kind/readme.md), with one real deviation — `Money` and `Price` are the same type today (`Decimal<MONEY_SCALE>`), not two independent structs; `Price` has no consumer anywhere that needs it distinguished from `Money` yet. `Quantity` is the genuinely distinct kind, wrapping `Decimal` and refusing negative values. See [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) and [its non-negativity decision](../../module/exact_kind/docs/decisions/001_non_negativity_enforced_at_construction.md) for the full account.

### Statement

- **Purpose**: money, quantity, and price are not the same type.
- **Why**: credits must not add to kilograms.
- **If missing**: silent unit mix.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:68-74` | Hard problem 3, verbatim Purpose/Why/If-missing |
