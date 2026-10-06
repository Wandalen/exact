# Scene: Price Snap Tick

### Scope

- **Purpose**: State the smoke lane's fourth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The tick-snap of a fixed price input.
- **In Scope**: The snap input, tick size, and expected output.
- **Out of Scope**: `exact_snap`'s general design (→ `../crate/012_exact_snap.md`, `../type/012_exact_snap_types.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — [`price_snap_tick`](../../module/exact_snap/readme.md) snaps `"1.26"` to a `"0.05"` tick under `Rounding::Down` and the result is asserted equal to `"1.25"`; the lane prints `tick=1.25`. The function takes the tick as a `Tick` and an explicit rounding mode (→ [`type/012_exact_snap_types.md`](../type/012_exact_snap_types.md)).

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
