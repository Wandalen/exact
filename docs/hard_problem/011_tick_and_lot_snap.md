# Hard Problem: Tick And Lot Snap

### Scope

- **Purpose**: State why price/quantity grid-snapping belongs in this family, not in the order book.
- **Responsibility**: The requirement that only grid-legal prices/quantities ever reach a book.
- **In Scope**: Instrument tick/lot snapping.
- **Out of Scope**: The order book itself (workstream 002, outside this family).

**Design status**: implemented in [`exact_snap`](../../module/exact_snap/readme.md) as specified — `Tick(Price)`, `Lot(Quantity)`, `price_snap_tick`, and `qty_snap_lot` all match the proposal, with grid-spacing rounding driven by the shared `Rounding` enum from `exact_round` rather than a separate mode type, realizing the proposal's own framing. Holds as stated, with no real deviation.

### Statement

- **Purpose**: price and quantity snap to an instrument grid.
- **Why**: 002 must not reimplement this.
- **If missing**: illegal prices enter the book.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:132-138` | Hard problem 11, verbatim Purpose/Why/If-missing |
