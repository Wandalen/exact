# Type: exact_snap Types

### Scope

- **Purpose**: Specify `exact_snap`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Tick`/`Lot` and the snap functions.
- **In Scope**: The structs, functions, and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/012_exact_snap.md`).

**Design status**: implemented in [`exact_snap`](../../module/exact_snap/readme.md). `Tick`/`Lot` and `SnapError`'s three variants match as specified; the proposed third function `snap_mode` does not exist separately — `Rounding` (from `exact_round`) is instead a parameter directly on [`price_snap_tick`/`qty_snap_lot`](../../module/exact_snap/src/lib.rs), not a standalone mode-setting call. `Tick::new`/`Lot::new` refuse a zero-sized grid spacing (`SnapError::ZeroTick`/`ZeroLot`) but deliberately accept a negative one — see [`exact_snap`'s own invariant doc](../../module/exact_snap/docs/invariant/001_grid_spacing_never_zero.md) for why no variant exists for that case.

### Structs

- `Tick(Price)`
- `Lot(Qty)`

### Functions

- `price_snap_tick`, `qty_snap_lot` — the snap operations.
- `snap_mode` — uses `Rounding` from `exact_round`.

### Errors

- `SnapError { ZeroTick, ZeroLot, Overflow }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:490-494` | `exact_snap`'s full struct/function/error listing |
