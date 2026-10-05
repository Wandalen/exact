# Scene: Add Subtract Assert Sum Zero

### Scope

- **Purpose**: State the smoke lane's second step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The add/subtract/assert-zero sequence over the two parsed values.
- **In Scope**: The arithmetic sequence and its exact-zero pass criterion.
- **Out of Scope**: The parse step that produces these values (→ `001_parse_money_at_scale_two.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — the two scale-2 values are added with `checked_add`, `"13.33"` is subtracted with `checked_sub`, and the result is asserted equal to `Decimal::< 2 >::ZERO`; the lane prints `sum=0`. The per-kind `money_add`/`money_sub` of [`exact_add`](../../module/exact_add/readme.md) take `Money` (scale 6) only, so the scale-2 values use `Decimal`'s own checked methods, which those functions dispatch to.

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
