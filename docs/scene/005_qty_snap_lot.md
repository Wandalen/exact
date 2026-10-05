# Scene: Qty Snap Lot

### Scope

- **Purpose**: State the smoke lane's fifth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The lot-snap of a fixed quantity input.
- **In Scope**: The snap input, lot size, and expected output.
- **Out of Scope**: `exact_snap`'s general design (→ `../crate/012_exact_snap.md`, `../type/012_exact_snap_types.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — [`qty_snap_lot`](../../module/exact_snap/readme.md) snaps `10` to a lot of `3` under `Rounding::Down` and the result is asserted equal to `9`; the lane prints `lot=9`. The function takes the lot as a `Lot` and an explicit rounding mode (→ [`type/012_exact_snap_types.md`](../type/012_exact_snap_types.md)).

### Procedure

1. Call `qty_snap_lot` on `10` with lot `3`.
2. Assert the result is `9`.

### Pass Criterion

Golden print `lot=9`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:534` | "qty_snap_lot of 10 to lot 3 → 9." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:552,558` | Golden print `tick=1.25 lot=9`; "lot=9" |
