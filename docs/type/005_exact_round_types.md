# Type: exact_round Types

### Scope

- **Purpose**: Specify `exact_round`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: The `Rounding` enum and its default/name functions.
- **In Scope**: The enum and functions this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/005_exact_round.md`).

**Design status**: implemented as specified in [`exact_round`](../../module/exact_round/readme.md) — `Rounding { Down, Up, HalfEven }`, `rounding_name`, and `rounding_default` returning `Down`. Beyond this listing, the crate also owns `round_div`/`round_div_wide` and `RoundError`, the one shared rounding division `exact_ratio`, `exact_snap` and `exact_dust` call — see [ADR-002](../../module/exact_round/docs/decisions/002_round_div_owned_by_exact_round.md).

### Enums

- `Rounding { Down, Up, HalfEven }`

### Functions

- `rounding_default` — `Down`.
- `rounding_name` — human-readable name.

### Errors

None defined for this crate in the source.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:454-457` | `exact_round`'s full enum/function listing |
