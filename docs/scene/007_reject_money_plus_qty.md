# Scene: Reject Money Plus Qty

### Scope

- **Purpose**: State the smoke lane's seventh step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: Demonstrating kind-mixing rejection by API absence rather than by a runtime check.
- **In Scope**: What this step actually demonstrates (no callable path exists) and why it has no golden-print line of its own.
- **Out of Scope**: `exact_kind`'s general design (→ `../crate/003_exact_kind.md`, `../type/003_exact_kind_types.md`).

**Design status**: implemented as specified — the real [`exact_add`](../../module/exact_add/readme.md) ships exactly `money_add`, `qty_add`, `price_add` (and their `_sub` counterparts), each monomorphic over one kind; no `money_add_qty`-shaped function exists in the real surface either, and `Money` (`Decimal<MONEY_SCALE>`) and `Quantity` (`Qty<MONEY_SCALE>`) remain distinct Rust types a shared add cannot accept. This step's structural claim holds against the real API (→ [`type/006_exact_add_types.md`](../type/006_exact_add_types.md)); as a negative-space demonstration it has no demo-lane runtime step to check either way.

### Procedure

This step is a negative-space demonstration, not a runtime assertion: the smoke lane only ever calls the money-plus-money form (e.g. `money_add`), because no `money_add_qty`-shaped function exists in the proposed `exact_add`/`exact_kind` surface. The rejection is structural — there is no function to call, not a runtime error caught and reported.

### Why It Has No Golden-Print Line

Unlike the other 9 steps, this one proves a negative (kind mixing is unrepresentable) rather than producing a value, so there is nothing for the golden print to show — the absence itself is the evidence, checkable only by inspecting the API surface (`../type/`), not by running the lane.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:536` | "Reject money plus qty (Kind / no such function exists; the smoke only adds money to money)." |
