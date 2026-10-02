# Feature: Bytes Minor Plus Scale

### Scope

- **Purpose**: Wire-encode a value as integer plus scale, never as a float.
- **Responsibility**: `exact_bytes`'s `Wire` type and `_to_wire`/`_from_wire` functions.
- **In Scope**: `Wire { minor, scale, kind }` and the round-trip functions per kind.
- **Out of Scope**: Any project-wide file format (the source notes this is "not 012's file format").

**Design status**: implemented in [`exact_bytes`](../../module/exact_bytes/readme.md) — `Wire { minor: i64, scale: u8, kind: u8 }` and the `money_to_wire`/`money_from_wire` (and `qty_`/`price_`) round-trip functions match the proposal exactly. Diverges on `WireError`: it carries five variants, not the proposal's three (`BadKind`/`BadScale`/`Truncated`) — `Overflow` and `Negative { minor }` are added to cover a decoded `minor` that is in-range as a raw `i64` but still breaches a kind's declared ceiling or non-negativity rule — see `exact_bytes`'s own [decision](../../module/exact_bytes/docs/decisions/001_wire_error_overflow_and_negative_variants.md) for the full account.

### Statement

Serialization is `Wire { minor: i64, scale: u8, kind: u8 }` — an integer triple — never a floating-point field. Without this, a save file or network message reintroduces problem 1 (float money) the moment it touches disk or wire.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:194` | Feature 15 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:156-162` | Hard problem 14, "Serialization," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:484-489` | `exact_bytes`'s proposed `Wire` type and `WireError` |
