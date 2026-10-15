# Type: exact_scale Types

### Scope

- **Purpose**: Specify `exact_scale`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Scale` and the scale-factor/equality functions.
- **In Scope**: Structs, constants, functions, and the error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/002_exact_scale.md`).

**Design status**: implemented in [`exact_scale`](../../module/exact_scale/readme.md), which rejects this proposal's central design: scale is a compile-time `const` generic on `exact_kind::Decimal<SCALE>`, never a runtime `Scale(u8)` value — see [its own `docs/readme.md`](../../module/exact_scale/docs/readme.md) and [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) for where that mechanism actually lives. None of `Scale`, `SCALE_MAX`, `scale_new`, `scale_get`, `scale_eq`, `scale_factor`, or `ScaleError` exist. In their place: `MONEY_SCALE`, `CEILING_WHOLE_UNITS`, `CEILING_MINOR_UNITS`, `HEADROOM_FACTOR` constants and a public `pow10` function — see [Representable Range And Headroom](../../module/exact_scale/docs/non_functional_requirement/001_representable_range_and_headroom.md) for the full account. The "power-of-ten table lives here, not the facade" placement call was followed.

### Structs and Constants

- `Scale(u8)` — the scale type.
- `SCALE_MAX` — the declared upper bound.

### Functions

- `scale_new`, `scale_get` — construction and access.
- `scale_eq` — same-scale check.
- `scale_factor` — `10^scale`, checked.

### Errors

- `ScaleError { TooLarge, FactorOverflow }`

### Private Detail

The power-of-ten table lives here, not in the facade.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:434-439` | `exact_scale`'s full struct/function/error listing |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:509` | "the power-of-ten table lives in exact_scale" |
