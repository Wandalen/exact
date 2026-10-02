# Hard Problem: Scale Mismatch

### Scope

- **Purpose**: State why operating across two scales must be an explicit error, not a silent raw operation.
- **Responsibility**: The requirement that mismatched-scale operations never proceed unconverted.
- **In Scope**: Any operation spanning two values of different declared scale.
- **Out of Scope**: The conversion function itself (→ `../feature/019_scale_convert_explicit.md`).

**Design status**: resolved by a design choice rather than handled at runtime. The proposed [`scale_convert`/`ScaleError` surface (feature 019)](../feature/019_scale_convert_explicit.md) was never built, because it became unnecessary — [`exact_kind`](../../module/exact_kind/readme.md) makes `SCALE` a `const` generic on `Decimal<SCALE>`/`Qty<SCALE>`, so two different scales are two different, incompatible Rust types, and mixing them is a compiler error, not a runtime condition any `scale_convert` call or `ScaleError` could ever be reached to report. The hard problem is avoided by construction rather than solved by a checked operation — see [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) for the mechanism.

### Statement

- **Purpose**: operating on two scales is an error unless an explicit convert exists.
- **Why**: 2-digit cash and 8-digit commodity must not add raw.
- **If missing**: shifted magnitudes.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:148-154` | Hard problem 13, verbatim Purpose/Why/If-missing |
