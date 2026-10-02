# Type: exact_round Types

### Scope

- **Purpose**: Specify `exact_round`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: The `Rounding` enum and its default/name functions.
- **In Scope**: The enum and functions this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/005_exact_round.md`).

**Design status**: implemented in [`exact_round`](../../module/exact_round/readme.md). `Rounding { Down, Up, HalfEven }` and `rounding_name` match this proposal exactly. `rounding_default` deviates in its return value — it returns `HalfEven`, not `Down` as specified here — see [Half-Even As The Default](../../module/exact_round/docs/decisions/001_half_even_as_the_unbiased_default.md) for why. This proposal's "no errors defined" is now false: the real crate adds `RoundError { DivZero, Overflow }` for `round_div`, a rounding-division function this proposal never named for this crate — see [`round_div` Owned By `exact_round`](../../module/exact_round/docs/decisions/002_round_div_owned_by_exact_round.md) and [Rounding Mode](../../module/exact_round/docs/type/001_rounding_mode.md).

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
