# Hard Problem: Dust Destination

### Scope

- **Purpose**: State why the leftover subunit from a split needs a defined home.
- **Responsibility**: The requirement that rounding's remainder never simply vanish.
- **In Scope**: Split/dust arithmetic.
- **Out of Scope**: The destination mechanism itself (→ `../feature/010_remainder_assign_dust_destination.md`).

**Design status**: implemented in [`exact_dust`](../../module/exact_dust/readme.md), matching the proposal's `DustTo{First, Sink, Reject}` and `DustError{EmptyParts, Remainder, Overflow}` shapes exactly — the requirement holds as stated. Naming deviates to this family's per-kind dispatch convention (`money_dust_split`/`qty_dust_split` rather than one generic `dust_split`), and one gap in the original proposal was filled: a `DustTo::First` correction that would push a non-negative `Quantity` below zero is reported as `Overflow` — see [`exact_dust`'s own decision](../../module/exact_dust/docs/decisions/003_leftover_via_raw_minor_reconstruction.md) for the full account.

### Statement

- **Purpose**: the last subunit has a destination (fee sink or first party).
- **Why**: rounding leaves a remainder.
- **If missing**: one subunit vanishes forever.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:92-98` | Hard problem 6, verbatim Purpose/Why/If-missing |
