# Scene: Checked Add Overflow At Max

### Scope

- **Purpose**: State the smoke lane's eighth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The overflow-at-boundary check for checked addition.
- **In Scope**: The boundary input and expected error, not a wrapped value.
- **Out of Scope**: `exact_add`'s general design (→ `../crate/006_exact_add.md`, `../type/006_exact_add_types.md`).

**Design status**: not exercised by the demo lane — every `checked_add` call in [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) stays far below the ceiling and `.expect`s success; no overflow path runs. The underlying capability is real but renamed: there is no `AddError` — `checked_add` returns `exact_kind::KindError` directly, and this step's overflow-not-wrap case is `KindError::Overflow { operation }` (→ [`exact_add`](../../module/exact_add/readme.md), [`type/006_exact_add_types.md`](../type/006_exact_add_types.md)).

### Procedure

1. Perform a checked add at `i64::MAX`.
2. Assert the result is `AddError::Overflow` — never a silently wrapped value.

### Pass Criterion

Golden print `overflow=1` (a boolean-style flag confirming the overflow was caught, not wrapped).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:537` | "Checked add at i64::MAX → Overflow, not wrap." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:553,558` | Golden print `extra=1 overflow=1`; "overflow=1" |
