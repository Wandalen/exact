# Hard Problem: One Canonical Subunit

### Scope

- **Purpose**: State why every value needs one shared subunit rather than a display convention.
- **Responsibility**: The requirement that storage be cents/grams/milli-units, never a display string.
- **In Scope**: The scale every caller must agree on.
- **Out of Scope**: The scale type itself (→ `../feature/002_scale_digits_after_point.md`).

**Design status**: implemented across [`exact_scale`](../../module/exact_scale/readme.md) and [`exact_kind`](../../module/exact_kind/readme.md), but not via the proposed mechanism — scale is a compile-time `const SCALE: u32` generic on `exact_kind::Decimal`/`Qty`, not a runtime `Scale` value every caller passes around, and `exact_scale` itself holds only the `MONEY_SCALE`/ceiling constants and `pow10`, never a `Scale` type. The actual requirement (one shared subunit, not a display convention) is satisfied more strongly than proposed: two different scales are two different, incompatible Rust types, so sharing the wrong scale is a compile error rather than a runtime discipline — see [`exact_scale` Types](../type/002_exact_scale_types.md) and [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) for the full account.

### Statement

- **Purpose**: store cents, grams, or milli-units, not a display string.
- **Why**: every caller must share a scale.
- **If missing**: mixed 2-digit and 8-digit bugs.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:60-66` | Hard problem 2, verbatim Purpose/Why/If-missing |
