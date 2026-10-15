# Feature: Conservation Error

### Scope

- **Purpose**: Report a failed conservation fold with the actual discrepancy, not a bare boolean.
- **Responsibility**: `exact_conserve`'s `ConservationError` type.
- **In Scope**: `ConservationError::NotZero { got }` and `ConservationError::Overflow`.
- **Out of Scope**: The fold itself (→ Feature 013).

**Design status**: implemented in [`exact_conserve`](../../module/exact_conserve/readme.md) as specified — `ConservationError::NotZero { got: i128 }` and `ConservationError::Overflow` match the proposal's shape exactly; the proposal left `got`'s type unspecified, and `exact_conserve` fills it in as `i128` to match `Report::discrepancy_minor` and to uniformly cover `Quantity`'s non-negative representation — see `exact_conserve`'s own [type doc](../../module/exact_conserve/docs/type/001_entry_and_report.md) for the full account.

### Statement

A failed `sum_assert_zero` returns `ConservationError::NotZero { got }`, carrying the actual non-zero net rather than just signaling failure — the discrepancy amount is itself the diagnostic.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:192` | Feature 14 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:498` | `ConservationError { NotZero { got }, Overflow }` |
