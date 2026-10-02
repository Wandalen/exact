# Scene: Split With Dust To Sink

### Scope

- **Purpose**: State the smoke lane's third step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The 3-way split of `10.00`, its rounding mode, and dust destination.
- **In Scope**: The split inputs, mode, and the parts-plus-dust conservation check.
- **Out of Scope**: `exact_dust`'s general design (→ `../crate/008_exact_dust.md`, `../type/008_exact_dust_types.md`).

**Design status**: implemented, but the demo lane exercises a different `DustTo` variant and different literals — step 5 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) calls the real [`money_dust_split`](../../module/exact_dust/readme.md)`(fill, parts, Rounding::Down, DustTo::First)` on `"100.000001"` into 3 parts (dust folded into the first share, not reported separately), not this step's `"10.00"`-into-3 / `DustTo::Sink` / separate-dust-line scenario. `DustTo::Sink` is itself real and unchanged by name (→ [`type/008_exact_dust_types.md`](../type/008_exact_dust_types.md)) — just not the variant this particular demo run uses.

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
