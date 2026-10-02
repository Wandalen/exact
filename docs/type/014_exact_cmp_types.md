# Type: exact_cmp Types

### Scope

- **Purpose**: Specify `exact_cmp`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: Comparison, equality, and min/max per kind.
- **In Scope**: The functions and error enum this crate would define, and the conditional-`Ord` rule.
- **Out of Scope**: Its dependency edges (→ `../crate/014_exact_cmp.md`).

**Design status**: implemented in [`exact_cmp`](../../module/exact_cmp/readme.md). The six comparison/equality/extrema functions match as specified; it defines no `CmpError` and implements `Ord` unconditionally rather than after a scale check — a scale mismatch is a compile error under this family's const-generic `Decimal<SCALE>`, never a runtime value these functions could receive — see [`exact_cmp`'s own decision](../../module/exact_cmp/docs/decisions/001_no_cmp_error_unconditional_ord.md) for the full account.

### Functions

- `money_cmp`, `qty_cmp`, `price_cmp` — ordering per kind.
- `money_eq`, `price_min`, `price_max` — equality and extrema.

### Errors

- `CmpError { ScaleMismatch }`

### Design Rule

`Ord` is implemented only after a scale check; the source explicitly warns: do not implement `Ord` if scales may differ without a documented same-scale invariant.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:499-503` | `exact_cmp`'s full function/error listing, including the conditional-`Ord` warning |
