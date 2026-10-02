# Feature: Rounding Mode Enum

### Scope

- **Purpose**: Make the rounding mode a value every caller must name, rather than an implicit per-caller convention.
- **Responsibility**: `exact_round`'s `Rounding` enum.
- **In Scope**: `Down`, `Up`, `HalfEven` variants and a documented default.
- **Out of Scope**: The decimal type the mode applies to (`exact_round` has no dependents on a decimal type itself — it's consumed by `exact_ratio`/`exact_snap`).

**Design status**: implemented in [`exact_round`](../../module/exact_round/readme.md) — the `Rounding` enum's three variants (`Down`, `Up`, `HalfEven`) match this proposal exactly. The documented default deviates: `rounding_default()` returns `HalfEven`, not `Down` as originally planned, chosen because it is the one mode with no directional bias over a long run of roundings — see [`exact_round`'s own decision](../../module/exact_round/docs/decisions/001_half_even_as_the_unbiased_default.md) for the full account.

### Statement

`Rounding` is data — `Down`, `Up`, or `HalfEven` — passed explicitly into every division or snap call. Without a shared enum, each caller truncates differently and conservation breaks on the remainder.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:180` | Feature 8 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:84-90` | Hard problem 5, "Division and rounding," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:454-457` | `exact_round`'s proposed enum and functions |
