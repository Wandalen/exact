# docs

Design documentation for `exact_sign`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `type/` | The sign-classification vocabulary — `Sign`, and the functions classifying a `Backing` value against it |
| `invariant/` | Totality and mutual exclusivity of the three-way sign classification |
| `decisions/` | Why negative-admission is a policy function taking an explicit bool, not a per-kind trait |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

This is Tier 1 of the family, depending on `exact_minor` alone for the
backing primitive a sign is classified over. Net-new: the family's prior
shape encoded "can this go negative" as a type-level choice rather than a
runtime policy value, so nothing here ports old code — every item is written
fresh against the current source. See
[the policy-function decision](decisions/001_negative_admission_as_a_policy_function.md)
for where this crate's own functions are (and are not yet) actually consumed
elsewhere in the family.
