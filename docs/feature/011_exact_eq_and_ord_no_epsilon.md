# Feature: Exact Eq And Ord No Epsilon

### Scope

- **Purpose**: Compare values by exact integer equality, never by a tolerance window.
- **Responsibility**: `exact_cmp`'s comparison functions.
- **In Scope**: `money_cmp`/`money_eq`, `qty_cmp`, `price_cmp`/`price_min`/`price_max`.
- **Out of Scope**: `Ord` on a type whose scale may differ without a documented same-scale invariant — the source explicitly forbids implementing it in that case.

**Design status**: implemented in [`exact_cmp`](../../module/exact_cmp/readme.md) — `money_cmp`/`money_eq`, `qty_cmp`, `price_cmp`/`price_min`/`price_max` all exist under these exact names, dispatching to bit-exact integer comparison with no tolerance. Diverges from the proposal's own caution against unconditional `Ord`: there is no `CmpError` and no scale guard — `exact_kind`'s `Ord`/`PartialEq` are derived unconditionally, because this family's compile-time `SCALE` generic makes a scale mismatch a compile error no runtime check could ever reach — see [`exact_cmp`'s own decision](../../module/exact_cmp/docs/decisions/001_no_cmp_error_unconditional_ord.md) for the full account.

### Statement

Comparison is bit-exact integer comparison after a scale check, never a float compare with an epsilon. A book that compares with tolerance is a book where two prices can be "equal" and "different" depending on which comparison ran.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:186` | Feature 11 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:100-106` | Hard problem 7, "Determinism," this feature supports |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:499-503` | `exact_cmp`'s proposed functions and `CmpError` |
