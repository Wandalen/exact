# Scene: Parse Money At Scale Two

### Scope

- **Purpose**: State the smoke lane's first step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: Parsing the two fixed input strings the rest of the scenario builds on.
- **In Scope**: The parse inputs and target type/scale.
- **Out of Scope**: The add/subtract step that consumes these values (→ `002_add_subtract_assert_sum_zero.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — `"10.00"` and `"3.33"` are parsed as `Decimal< 2 >`, the scale-2 instance of the type `Money` (`Decimal< 6 >`) also instantiates. `Money` itself has no selectable scale: the family shares one fixed [`MONEY_SCALE = 6`](../../module/exact_scale/readme.md), so the scale-2 value is a `Decimal< 2 >` rather than a `Money`.

### Procedure

1. Parse the literal `"10.00"` as `Money` at scale 2.
2. Parse the literal `"3.33"` as `Money` at scale 2.

Both are fixed constants — no seed, no randomness anywhere in this lane.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:530` | "Parse "10.00" and "3.33" as Money at scale 2." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:527` | "Headless. One file. No book, no wallets. Seed is irrelevant; all inputs are constants." |
