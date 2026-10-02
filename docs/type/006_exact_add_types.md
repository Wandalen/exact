# Type: exact_add Types

### Scope

- **Purpose**: Specify `exact_add`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: Checked/saturating add and sub per kind.
- **In Scope**: The functions and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/006_exact_add.md`).

**Design status**: implemented in [`exact_add`](../../module/exact_add/readme.md). All six functions (`money`/`qty`/`price_add`/`sub`), both saturating variants, and `money_checked_neg` match this proposal's names exactly. Diverges on the error type — there is no `AddError`; every function returns `exact_kind::KindError` directly, since none of `AddError`'s three proposed variants (`Overflow`, `ScaleMismatch`, `NegNotAllowed`) would have a reachable construction site distinct from what `KindError` already covers — see [`exact_add`'s decision](../../module/exact_add/docs/decisions/001_reuse_kinderror_no_wrapper_type.md) for the full account.

### Functions

- `money_add`, `money_sub`, `qty_add`, `qty_sub`, `price_add`, `price_sub` — checked arithmetic per kind.
- `money_saturating_add`, `qty_saturating_add` — saturating variants.
- `money_checked_neg` — negation where allowed.

### Errors

- `AddError { Overflow, ScaleMismatch, NegNotAllowed }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:458-462` | `exact_add`'s full function/error listing |
