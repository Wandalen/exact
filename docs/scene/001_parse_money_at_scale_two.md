# Scene: Parse Money At Scale Two

### Scope

- **Purpose**: State the smoke lane's first step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: Parsing the two fixed input strings the rest of the scenario builds on.
- **In Scope**: The parse inputs and target type/scale.
- **Out of Scope**: The add/subtract step that consumes these values (→ `002_add_subtract_assert_sum_zero.md`).

**Design status**: implemented, but not as specified — `Money` has no selectable "scale 2": the whole family shares one fixed [`MONEY_SCALE = 6`](../../module/exact_scale/readme.md), so this literal pair ("10.00"/"3.33" at scale 2) is unrepresentable as written. The demo lane, [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md), does exercise this step's underlying capability — `Money::parse("0.1")`, the inherent `Decimal::parse` in [`exact_kind`](../../module/exact_kind/readme.md) — just with a different literal and the family's one fixed scale, not these two inputs.

### Procedure

1. Parse the literal `"10.00"` as `Money` at scale 2.
2. Parse the literal `"3.33"` as `Money` at scale 2.

Both are fixed constants — no seed, no randomness anywhere in this lane.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:530` | "Parse "10.00" and "3.33" as Money at scale 2." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:527` | "Headless. One file. No book, no wallets. Seed is irrelevant; all inputs are constants." |
