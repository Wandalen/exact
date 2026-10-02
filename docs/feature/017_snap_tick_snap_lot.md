# Feature: Snap Tick Snap Lot

### Scope

- **Purpose**: Snap a price or quantity onto an instrument's grid before it enters a book.
- **Responsibility**: `exact_snap`'s `price_snap_tick`/`qty_snap_lot` functions.
- **In Scope**: `Tick(Price)`, `Lot(Qty)`, and `snap_mode` (driven by `Rounding`).
- **Out of Scope**: The order book that consumes snapped values (workstream 002, out of this family entirely).

**Design status**: implemented in [`exact_snap`](../../module/exact_snap/readme.md) as specified — `Tick(Price)`, `Lot(Quantity)`, `price_snap_tick`, and `qty_snap_lot` all match the proposal; the grid-spacing `rounding: Rounding` parameter is driven by the same `Rounding` enum the rest of the family shares (`exact_round`), realizing the proposal's own "driven by `Rounding`" framing of `snap_mode` rather than naming a separate mode type.

### Statement

`price_snap_tick`/`qty_snap_lot` round a value onto its instrument's tick/lot grid, so workstream 002 never has to reimplement grid-snapping itself and illegal (off-grid) prices never reach the book.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:198` | Feature 17 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:132-138` | Hard problem 11, "Tick and lot snap," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:490-494` | `exact_snap`'s proposed types, functions, and `SnapError` |
