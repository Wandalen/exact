# Scene: Checksum Equality Across Calls

### Scope

- **Purpose**: State the smoke lane's tenth and final step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The determinism check — two independent calls must agree bit-for-bit.
- **In Scope**: The two-call comparison and its pass criterion.
- **Out of Scope**: The family-wide determinism requirement this step demonstrates (→ `../hard_problem/007_determinism.md`, built by a parallel collection).

**Design status**: not implemented anywhere in the real family, and not exercised by the demo lane — no crate in `module/` computes a checksum of minors; "checksum" appears only in the archived proposal source and this corpus. The real family demonstrates determinism differently: [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) asserts its exact arm and an `f64` control arm *disagree* on the same computation (not that two exact calls agree), and [`exact_cmp`](../../module/exact_cmp/readme.md) provides the family's epsilon-free `Eq`/`Ord` (→ [`../hard_problem/007_determinism.md`](../hard_problem/007_determinism.md)).

### Procedure

1. Run the scenario's checksum-of-minors computation once, call it `a`.
2. Run it again, call it `b`.
3. Assert `a == b`.

### Pass Criterion

Golden print `a=0x… b=0x…` followed by `ok`; pass condition is explicitly `a==b`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:539` | "Two calls, same checksum of minors." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:555-558` | Golden print `a=0x… b=0x…` / `ok`; "a==b" |
