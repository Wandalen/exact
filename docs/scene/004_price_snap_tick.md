# Scene: Price Snap Tick

### Scope

- **Purpose**: State the smoke lane's fourth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The tick-snap of a fixed price input.
- **In Scope**: The snap input, tick size, and expected output.
- **Out of Scope**: `exact_snap`'s general design (→ `../crate/012_exact_snap.md`, `../type/012_exact_snap_types.md`).

**Design status**: not exercised by the demo lane — [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md)'s 5 steps never call a snap function. The function itself is real and matches this proposal's name exactly, [`price_snap_tick`](../../module/exact_snap/readme.md) in `exact_snap` (→ [`type/012_exact_snap_types.md`](../type/012_exact_snap_types.md)) — just not demoed in this lane.

### Procedure

1. Call `price_snap_tick` on `"1.26"` with tick `"0.05"`.
2. Assert the result is `"1.25"`.

### Pass Criterion

Golden print `tick=1.25`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:533` | "price_snap_tick of "1.26" to tick "0.05" → "1.25"." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:552,558` | Golden print `tick=1.25 lot=9`; "tick=1.25" |
