# Feature: Remainder Assign Dust Destination

### Scope

- **Purpose**: Give the leftover subunit from a split a named home instead of letting it vanish.
- **Responsibility**: `exact_dust`'s remainder-assignment functions.
- **In Scope**: `dust_remainder`, `dust_split`/`dust_split_into`, and the `DustTo` destination enum.
- **Out of Scope**: The split's multiplication itself (→ Feature 007, `exact_ratio`).

**Design status**: implemented in [`exact_dust`](../../module/exact_dust/readme.md), matching the proposal's `DustTo{First, Sink, Reject}` and `DustError{EmptyParts, Remainder, Overflow}` exactly. The function surface deviates in naming: the proposal names one generic `dust_split`/`dust_remainder`/`dust_split_into` per operation, but the real crate follows this family's per-kind dispatch convention instead — `money_dust_split`/`qty_dust_split`, `money_dust_remainder`/`qty_dust_remainder`, `money_dust_split_into`/`qty_dust_split_into`. One further filled gap: when a `DustTo::First` correction would push a non-negative `Quantity` below zero, it is reported as `Overflow` rather than a dedicated variant — see [`exact_dust`'s own decision](../../module/exact_dust/docs/decisions/003_leftover_via_raw_minor_reconstruction.md) for the full account.

### Statement

Rounding a split leaves a remainder; `remainder_assign` routes it to a named destination (`First`, `Sink`, or `Reject`) rather than letting it silently disappear. This is what keeps parts-plus-dust exactly equal to the original total.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:184` | Feature 10 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:92-98` | Hard problem 6, "Dust," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:469-474` | `exact_dust`'s proposed types, functions, and `DustError` |
