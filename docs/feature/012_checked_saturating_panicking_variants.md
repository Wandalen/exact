# Feature: Checked Saturating Panicking Variants

### Scope

- **Purpose**: Give every overflow-prone operation three explicitly named policies instead of one implicit behavior.
- **Responsibility**: The `_checked_*`/`_saturating_*` naming convention across `exact_minor` and `exact_add`.
- **In Scope**: `minor_checked_add`/`minor_saturating_add`, `money_saturating_add`, `qty_saturating_add`.
- **Out of Scope**: A panicking variant is named as a policy but not enumerated with functions in the source's crate-by-crate breakdown.

**Design status**: implemented in [`exact_minor`](../../module/exact_minor/readme.md) and [`exact_add`](../../module/exact_add/readme.md) — checked and saturating variants match the proposal exactly (`minor_checked_add`/`minor_checked_sub`/`minor_checked_neg`/`minor_saturating_add`/`minor_saturating_sub` in `exact_minor`; `money_saturating_add`/`qty_saturating_add` in `exact_add`). The panicking variant this feature names was deliberately not built — no real consumer calls for it — see `exact_add`'s own [no-panicking-variant decision](../../module/exact_add/docs/decisions/002_no_panicking_variant_yet.md) for the full account.

### Statement

Every arithmetic operation that can overflow gets named variants — checked (returns `Result`), saturating (clamps), and panicking (asserts) — so the caller's overflow policy is visible in the function name rather than hidden in a crate-wide default.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:188` | Feature 12 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:429-431` | `exact_minor`'s proposed checked/saturating function names |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:459-461` | `exact_add`'s proposed checked/saturating function names |
