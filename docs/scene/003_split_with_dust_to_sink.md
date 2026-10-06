# Scene: Split With Dust To Sink

### Scope

- **Purpose**: State the smoke lane's third step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The 3-way split of `10.00`, its rounding mode, and dust destination.
- **In Scope**: The split inputs, mode, and the parts-plus-dust conservation check.
- **Out of Scope**: `exact_dust`'s general design (→ `../crate/008_exact_dust.md`, `../type/008_exact_dust_types.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — `10` is split 3 ways under `Rounding::Down` with `DustTo::Sink` via [`money_dust_split`](../../module/exact_dust/readme.md), the held-back dust read with `money_dust_remainder`, and parts plus dust asserted equal to the total. It runs on `Money`, fixed at scale 6, because `exact_dust` takes `Money`/`Quantity` only — so the lane prints `parts=3.333333,3.333333,3.333333 dust=0.000001`, the scale-6 spelling of this step's scale-2 `3.33`/`0.01`. Step 5 separately splits a fill with `DustTo::First`.

### Procedure

1. Split `10.00` into 3 parts, rounding down.
2. Route the dust to the sink (`DustTo::Sink`).
3. Assert parts plus dust equal `10.00` — nothing vanishes.

### Pass Criterion

Golden print `parts=3.33,3.33,3.33 dust=0.01`, matching the fixture exactly.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:532` | "Split 10.00 into 3 parts, rounding down, dust to sink. Parts plus dust equal 10.00." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:551,558` | Golden print `parts=3.33,3.33,3.33 dust=0.01`; "dust line matches the fixture" |
