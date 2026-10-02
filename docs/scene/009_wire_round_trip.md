# Scene: Wire Round Trip

### Scope

- **Purpose**: State the smoke lane's ninth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The wire-encode/decode round trip for a fixed money value.
- **In Scope**: The round-trip input and the exact minor value it must reproduce.
- **Out of Scope**: `exact_bytes`'s general design (→ `../crate/011_exact_bytes.md`, `../type/011_exact_bytes_types.md`).

**Design status**: not exercised by the demo lane — [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) never encodes to or decodes from `Wire`. The functions are real and match this proposal's names exactly, [`money_to_wire`/`money_from_wire`](../../module/exact_bytes/readme.md) in `exact_bytes`, with `WireError` adding two variants beyond this proposal's three (→ [`type/011_exact_bytes_types.md`](../type/011_exact_bytes_types.md)) — just not demoed in this lane.

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
