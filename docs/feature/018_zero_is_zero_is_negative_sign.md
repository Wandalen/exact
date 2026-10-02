# Feature: Zero Is Zero Is Negative Sign

### Scope

- **Purpose**: Give sign and zero queries first-class, unambiguous operations.
- **Responsibility**: `exact_sign`'s query functions.
- **In Scope**: `sign_of`, `sign_is_negative`, `sign_is_zero`, `sign_neg_allowed`.
- **Out of Scope**: Negation itself (→ Feature 021).

**Design status**: implemented in [`exact_sign`](../../module/exact_sign/readme.md) — `sign_of` and `sign_neg_allowed` match the proposal's names exactly. `sign_is_negative`/`sign_is_zero` are realized as bare `is_negative`/`is_zero` (no `sign_` prefix) — see `exact_sign`'s own [type doc](../../module/exact_sign/docs/type/001_sign_classification.md) for the real names and behavior.

### Statement

Zero, positive, and negative are a defined three-way `Sign` enum with dedicated query functions, ruling out a "negative zero" case entirely rather than leaving it to the underlying integer's own representation.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:200` | Feature 18 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:116-122` | Hard problem 9, "Sign and zero," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:449-453` | `exact_sign`'s proposed enum, functions, and `SignError` |
