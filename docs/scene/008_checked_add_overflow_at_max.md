# Scene: Checked Add Overflow At Max

### Scope

- **Purpose**: State the smoke lane's eighth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The overflow-at-boundary check for checked addition.
- **In Scope**: The boundary input and expected error, not a wrapped value.
- **Out of Scope**: `exact_add`'s general design (→ `../crate/006_exact_add.md`, `../type/006_exact_add_types.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — `minor_checked_add` of [`exact_minor`](../../module/exact_minor/readme.md) adds `1` to `i64::MAX` and the refusal is asserted; the lane prints `overflow=1`. The add runs on the raw `Minor`, because a `Money` cannot hold `i64::MAX` at all — its declared ceiling refuses far below it. The error is renamed: there is no `AddError` — the refusal is `MinorError::Overflow`.

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
