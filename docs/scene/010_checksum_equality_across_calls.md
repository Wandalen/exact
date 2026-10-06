# Scene: Checksum Equality Across Calls

### Scope

- **Purpose**: State the smoke lane's tenth and final step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The determinism check — two independent calls must agree bit-for-bit.
- **In Scope**: The two-call comparison and its pass criterion.
- **Out of Scope**: The family-wide determinism requirement this step demonstrates (→ `../hard_problem/007_determinism.md`, built by a parallel collection).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) — `checksum` folds every minor-unit value the scenes produced through FNV-1a, the scenes are run twice, and the two checksums are asserted equal; the lane prints `a=0x… b=0x…` and `ok`. The checksum lives in the lane itself, not in a family crate — no crate in `module/` computes one. `tests/lane_test.rs`'s `the_checksum_is_stable_across_calls_and_changes_with_any_value` also proves it changes when any value changes.

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
