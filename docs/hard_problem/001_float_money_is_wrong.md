# Hard Problem: Float Money Is Wrong

### Scope

- **Purpose**: State why binary floats are disqualified as a representation for money.
- **Responsibility**: The requirement that no conserved value ever be a float.
- **In Scope**: Money, quantity, and price representation.
- **Out of Scope**: Which crate/feature enforces this (→ `../crate/`, `../feature/`).

**Design status**: implemented in [`exact_minor`](../../module/exact_minor/readme.md) and [`exact_kind`](../../module/exact_kind/readme.md), holding exactly as stated with no deviation. Money is stored as a plain integer `Backing` (`i64`) at the base, and `Decimal`/`Qty` never admit a float anywhere in their public construction, inspection, or rendering surface — enforced by each crate's own dedicated invariant: [No Float In Representation](../../module/exact_minor/docs/invariant/001_no_float_in_representation.md) and [No Float In The Public Constructor Surface](../../module/exact_kind/docs/invariant/001_no_float_in_the_public_constructor_surface.md).

### Statement

- **Purpose**: `0.1 + 0.2` must not run a market.
- **Why**: binary floats cannot represent most decimal amounts.
- **If missing**: rounding exploits and failed audits.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:52-58` | Hard problem 1, verbatim Purpose/Why/If-missing |
