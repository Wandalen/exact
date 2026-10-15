# Type: exact_sign Types

### Scope

- **Purpose**: Specify `exact_sign`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: The `Sign` enum and sign-query functions.
- **In Scope**: The enum, functions, and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/004_exact_sign.md`).

**Design status**: implemented in [`exact_sign`](../../module/exact_sign/readme.md), matching closely except for `SignError`. `Sign { Neg, Zero, Pos }` and `sign_of` match exactly. `sign_is_negative` and `sign_is_zero` match exactly. `sign_neg_allowed(neg_allowed: bool, value: Backing) -> bool` matches this proposal's name and shape — see [Negative-Admission As A Policy Function](../../module/exact_sign/docs/decisions/001_negative_admission_as_a_policy_function.md) for why it takes the policy as an explicit parameter rather than being hard-coded per kind. No `SignError` exists: `sign_neg_allowed` is a pure predicate returning `bool`, never a `Result`, so there is nothing for a `NegNotAllowed` variant to report. A negative quantity is refused by `exact_kind::KindError::Negative` instead.

### Enums

- `Sign { Neg, Zero, Pos }`

### Functions

- `sign_of`, `sign_is_negative`, `sign_is_zero` — classification.
- `sign_neg_allowed` — money and price allow it; quantity is a policy flag.

### Errors

- `SignError { NegNotAllowed }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:449-453` | `exact_sign`'s full enum/function/error listing |
