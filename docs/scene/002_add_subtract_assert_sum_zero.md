# Scene: Add Subtract Assert Sum Zero

### Scope

- **Purpose**: State the smoke lane's second step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The add/subtract/assert-zero sequence over the two parsed values.
- **In Scope**: The arithmetic sequence and its exact-zero pass criterion.
- **Out of Scope**: The parse step that produces these values (→ `001_parse_money_at_scale_two.md`).

**Design status**: implemented, but not with this shape or these literals — the demo lane's nearest counterpart is step 2 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md): ten `0.1`s summed via `checked_add` and asserted equal to `Money::parse("1.0")`, not this step's add-10.00-then-subtract-13.33-assert-zero sequence. A function named almost exactly for this step's concept, `money_sum_assert_zero`, is real and ships in [`exact_conserve`](../../module/exact_conserve/readme.md), but the demo lane doesn't call it — its own zero-sum check (step 4) instead runs `verify()`/`Report::is_balanced()` over ledger `Entry` records, a related but differently-shaped conservation check.

### Procedure

1. Add the two parsed values (`10.00 + 3.33 = 13.33`).
2. Subtract `13.33`.
3. Assert the result is exactly zero.

### Pass Criterion

`sum=0` in the golden print — exact equality, not within-tolerance.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:531` | "Add them. Subtract "13.33". Sum must be zero." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:550,558` | Golden print `sum=0`; "Pass: sum=0, ..." |
