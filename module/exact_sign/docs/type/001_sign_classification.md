# Type: Sign Classification

### Scope

- **Purpose**: Define what `Sign` denotes and how a `Backing` value maps to it, so a caller classifies a value's sign without re-deriving the comparison at each call site.
- **Responsibility**: The `Sign` enum and the three classification functions built on it — `sign_of`, `is_negative`, `is_zero`.
- **In Scope**: Classification of a single `Backing` value.
- **Out of Scope**: What a classified sign is used *for* — the negative-admission policy built on top (→ [Negative-Admission As A Policy Function](../decisions/001_negative_admission_as_a_policy_function.md)).

### Definition

`Sign` is a three-variant enum — `Neg`, `Zero`, `Pos` — over a `Backing`
value's relationship to zero. `sign_of` is the sole classifier: strictly less
than zero is `Neg`, exactly zero is `Zero`, strictly greater than zero is
`Pos`. `is_negative` and `is_zero` are both expressed in terms of `sign_of`
rather than as independent comparisons, so there is exactly one place a sign
boundary could be drawn incorrectly.

This is net-new in the family: the prior shape encoded "can this go negative"
as a type-level choice (`Decimal` signed, `Qty` never) rather than a runtime
classification, so there is no real-code predecessor to port — every item
here is written fresh against the current source.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:16-26` | The `Sign` enum, its three variants |
| `src/lib.rs:29-58` | `sign_of` (29-44), `is_negative` (47-51), `is_zero` (54-58) |

### Tests

| File | Relationship |
|------|--------------|
| `tests/sign_classification_test.rs` | `sign_of_classifies_negative_zero_and_positive`, `is_negative_and_is_zero_agree_with_sign_of_at_the_boundary` |
