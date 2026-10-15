# Hard Problem: Overflow Handling

### Scope

- **Purpose**: State why overflow must be a named outcome, never a wrap.
- **Responsibility**: The requirement that add/multiply fail or follow a named policy on overflow.
- **In Scope**: All checked arithmetic across the family.
- **Out of Scope**: The specific checked/saturating functions (→ `../feature/006_checked_add_and_sub.md`, `../feature/012_checked_saturating_panicking_variants.md`).

**Design status**: implemented across [`exact_minor`](../../module/exact_minor/readme.md) (base checked/saturating primitives), [`exact_add`](../../module/exact_add/readme.md) (`money_add`/`money_sub` and the `qty`/`price` equivalents), and [`exact_ratio`](../../module/exact_ratio/readme.md) (checked multiply-by-ratio). Holds as stated — every path returns a typed error (`MinorError::Overflow` above the range or `MinorError::Underflow` below it, `exact_kind::KindError`, or `RatioError::Overflow`) rather than wrapping; see [`exact_minor`'s Checked Operations Total invariant](../../module/exact_minor/docs/invariant/002_checked_operations_total.md) for the base guarantee. One piece of the originally proposed policy surface was deliberately not built: the panicking variant, deferred as YAGNI since no real consumer calls for it — see [`exact_add`'s decision](../../module/exact_add/docs/decisions/002_no_panicking_variant_yet.md).

### Statement

- **Purpose**: add and multiply either fail or follow a named policy.
- **Why**: a wrap to negative wealth is a bug.
- **If missing**: silent wrap.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:76-82` | Hard problem 4, verbatim Purpose/Why/If-missing |
