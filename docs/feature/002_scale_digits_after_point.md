# Feature: Scale Digits After Point

### Scope

- **Purpose**: Carry "how many digits after the point" as explicit data, so no crate assumes a fixed scale.
- **Responsibility**: `exact_scale`'s primary type.
- **In Scope**: The `u8` scale value and same-scale comparison.
- **Out of Scope**: The kinds that carry a scale (→ `exact_kind`).

**Design status**: implemented, but not as proposed — scale is a compile-time `const SCALE: u32` generic parameter on [`exact_kind::Decimal<SCALE>`](../../module/exact_kind/docs/type/001_conserved_value_type_family.md), never a runtime `u8` value, and [`exact_scale`](../../module/exact_scale/readme.md) itself holds only the `pow10` table and ceiling constants, not a `Scale` type at all — see [`exact_scale` Types](../type/002_exact_scale_types.md) for the full account of the rejected runtime-`Scale` design.

### Statement

Scale (digits after the decimal point) is a `u8` value carried as data, not an implicit convention. Without it, callers silently assume "2 digits" and mixed 2-digit/8-digit amounts corrupt each other.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:168` | Feature 2 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:60-66` | Hard problem 2, "One canonical subunit," this feature addresses |
