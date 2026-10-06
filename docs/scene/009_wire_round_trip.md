# Scene: Wire Round Trip

### Scope

- **Purpose**: State the smoke lane's ninth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The wire-encode/decode round trip for a fixed money value.
- **In Scope**: The round-trip input and the exact minor value it must reproduce.
- **Out of Scope**: `exact_bytes`'s general design (→ `../crate/011_exact_bytes.md`, `../type/011_exact_bytes_types.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — `10` is encoded with [`money_to_wire`](../../module/exact_bytes/readme.md), decoded with `money_from_wire`, and asserted equal to the original. It runs on `Money`, fixed at scale 6, because `exact_bytes` takes `Money`/`Quantity`/`Price` only — so the lane prints `wire=10000000`, the scale-6 minor count of `10`, where this step's scale-2 encoding printed `1000`. `WireError` adds two variants beyond this proposal's three (→ [`type/011_exact_bytes_types.md`](../type/011_exact_bytes_types.md)).

### Procedure

1. Encode `"10.00"` to `Wire` via `money_to_wire`.
2. Decode it back via `money_from_wire`.
3. Assert the round-tripped value's minor equals the original minor.

### Pass Criterion

Golden print `wire=1000` — the scale-2 minor encoding of `10.00`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:538` | "Wire round-trip of "10.00" equals the original minor." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:554,558` | Golden print `wire=1000`; "wire=1000 (scale-2 minor of 10.00)" |
