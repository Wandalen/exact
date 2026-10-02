# Feature: Neg Only Where Kind Allows Debt

### Scope

- **Purpose**: Gate negation per-kind, so only kinds that can legitimately go negative expose it.
- **Responsibility**: `exact_sign`'s `sign_neg_allowed` policy check, consumed by `exact_add`'s `money_checked_neg`.
- **In Scope**: Money and price may go negative (debt, discount); quantity's negative-allowance is a policy flag.
- **Out of Scope**: The sign query functions themselves (→ Feature 018).

**Design status**: implemented, but substantially diverges from this proposal. `Money` and `Price` may go negative, matching the proposal — but `Quantity`'s non-negativity is not a policy flag: `exact_kind::Qty` refuses a negative value unconditionally at construction (`from_decimal`'s `value.minor() < 0` check), with no toggle. The named error also differs: there is no `SignError` type at all (`exact_sign` declares no error enum), and the real refusal is reported as `exact_kind::KindError::Negative { minor }`. `exact_sign::sign_neg_allowed` exists under the proposal's exact name but has no real caller — `exact_add::money_checked_neg` and `money_saturating_add` do not consume it — see `exact_kind`'s own [non-negativity decision](../../module/exact_kind/docs/decisions/001_non_negativity_enforced_at_construction.md) and `exact_sign`'s own [policy-function decision](../../module/exact_sign/docs/decisions/001_negative_admission_as_a_policy_function.md) for the full account.

### Statement

Negation is not universally available — `sign_neg_allowed` says which kinds may hold a negative value (money and price, yes; quantity, policy-gated), and a disallowed negation is a `SignError::NegNotAllowed`, not a silently-accepted negative quantity.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:206` | Feature 21 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:452-453` | `sign_neg_allowed` and `SignError { NegNotAllowed }` |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:461` | `exact_add`'s `money_checked_neg` |
