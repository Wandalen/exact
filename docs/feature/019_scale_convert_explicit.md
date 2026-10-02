# Feature: Scale Convert Explicit

### Scope

- **Purpose**: Make crossing between two scales an explicit, named call rather than an implicit operation.
- **Responsibility**: `exact_scale`'s `scale_eq`/conversion surface.
- **In Scope**: An explicit `scale_convert`-style call or a returned error when scales differ.
- **Out of Scope**: The scale type's own storage (→ Feature 002).

**Design status**: not built. No `scale_convert`-style function or `ScaleError` type exists anywhere in the real family — `exact_scale` holds only compile-time constants (`MONEY_SCALE`, the ceilings, `pow10`), never a runtime conversion or equality surface. The real family prevents scale mismatches at compile time instead: `SCALE` is a `const` generic on `Decimal`/`Qty`, so two different scales are two different, incompatible Rust types, and mixing them is a compiler error no runtime `scale_convert` call or `ScaleError` could ever be reached to report — see `exact_kind`'s own [type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) for the full account.

### Statement

Two values at different scales cannot be operated on directly — either an explicit conversion call bridges them, or the operation errors. This is what stops 2-digit cash and 8-digit commodity amounts from being added raw.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:202` | Feature 19 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:148-154` | Hard problem 13, "Scale mismatch," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:434-439` | `exact_scale`'s proposed functions and `ScaleError` |
