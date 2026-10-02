# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Sign` | enum | `src/lib.rs:16` | [Sign Classification](../type/001_sign_classification.md) |
| `sign_of` | fn | `src/lib.rs:30` | [Sign Classification](../type/001_sign_classification.md) |
| `is_negative` | fn | `src/lib.rs:48` | [Sign Classification](../type/001_sign_classification.md) |
| `is_zero` | fn | `src/lib.rs:55` | [Sign Classification](../type/001_sign_classification.md) |
| `sign_neg_allowed` | fn | `src/lib.rs:69` | [Negative-Admission As A Policy Function](../decisions/001_negative_admission_as_a_policy_function.md) |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
