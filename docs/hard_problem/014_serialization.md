# Hard Problem: Serialization

### Scope

- **Purpose**: State why wire and save formats must stay integer-plus-scale.
- **Responsibility**: The requirement that no serialized form ever carry a float for a conserved value.
- **In Scope**: Wire and save paths.
- **Out of Scope**: The `Wire` type itself (→ `../feature/015_bytes_minor_plus_scale.md`).

**Design status**: implemented in [`exact_bytes`](../../module/exact_bytes/readme.md) — `Wire { minor: i64, scale: u8, kind: u8 }` and the `money_to_wire`/`money_from_wire` (and `qty_`/`price_`) round-trip functions match the proposal exactly; holds as stated, integer-plus-scale with no float anywhere on the wire. `WireError` carries two more variants than proposed (`Overflow`, `Negative { minor }`), covering a decoded `minor` that is in-range as a raw `i64` but still breaches a kind's declared ceiling or non-negativity rule — see [`exact_bytes`'s own decision](../../module/exact_bytes/docs/decisions/001_wire_error_overflow_and_negative_variants.md) for the full account.

### Statement

- **Purpose**: wire and save are integer plus scale, not f64.
- **Why**: replay and 012.
- **If missing**: platform-dependent JSON.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:156-162` | Hard problem 14, verbatim Purpose/Why/If-missing |
