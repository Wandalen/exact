# Feature: Checked Add And Sub

### Scope

- **Purpose**: Make overflow a returned error instead of a silent wraparound.
- **Responsibility**: `exact_add`'s checked arithmetic functions.
- **In Scope**: `money_add`/`money_sub` and the `qty`/`price` equivalents.
- **Out of Scope**: Multiply/divide (→ `exact_ratio`).

**Design status**: implemented in [`exact_add`](../../module/exact_add/readme.md) as specified for the function surface — `money_add`/`money_sub` and the `qty`/`price` equivalents all exist under these exact names, returning a checked `Result` rather than wrapping. Diverges on the error type: there is no `AddError` — every function returns `exact_kind::KindError` directly, since none of the proposal's named `AddError` variants (`Overflow`, `ScaleMismatch`, `NegNotAllowed`) has a reachable construction site this crate's own enum would add — see [`exact_add`'s own decision](../../module/exact_add/docs/decisions/001_reuse_kinderror_no_wrapper_type.md) for the full account.

### Statement

Add and subtract return a `Result`, failing on overflow rather than wrapping to a negative or wrapped-around wealth value. A silent wrap here is exactly the kind of bug conservation checks exist to catch — better to never let it happen.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:176` | Feature 6 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:76-82` | Hard problem 4, "Overflow," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:458-462` | `exact_add`'s proposed functions and `AddError` |
