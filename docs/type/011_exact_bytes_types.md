# Type: exact_bytes Types

### Scope

- **Purpose**: Specify `exact_bytes`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Wire` and its to/from conversions per kind.
- **In Scope**: The struct, functions, and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/011_exact_bytes.md`).

**Design status**: implemented in [`exact_bytes`](../../module/exact_bytes/readme.md). `Wire`'s struct fields and all six to/from functions match as specified; `WireError` diverges — it adds `Overflow` and `Negative { minor }` beyond this proposal's three variants (`BadKind`, `BadScale`, `Truncated`) — see [`exact_bytes`'s own decision](../../module/exact_bytes/docs/decisions/001_wire_error_overflow_and_negative_variants.md) for the full account.

### Structs

- `Wire { minor: i64, scale: u8, kind: u8 }`

### Functions

- `money_to_wire`, `money_from_wire`
- `qty_to_wire`, `qty_from_wire`
- `price_to_wire`, `price_from_wire`

### Errors

- `WireError { BadKind, BadScale, Truncated }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:484-489` | `exact_bytes`'s full struct/function/error listing |
