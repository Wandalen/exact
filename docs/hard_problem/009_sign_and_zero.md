# Hard Problem: Sign And Zero

### Scope

- **Purpose**: State why sign and zero need an explicit, collapse-free definition.
- **Responsibility**: The requirement that zero, positive, and negative be well-defined with no negative zero.
- **In Scope**: Escrow and debt representation.
- **Out of Scope**: The `Sign` enum itself (→ `../feature/018_zero_is_zero_is_negative_sign.md`).

**Design status**: implemented in [`exact_sign`](../../module/exact_sign/readme.md) — the `Sign` enum (`Neg`/`Zero`/`Pos`) and `sign_of`/`is_negative`/`is_zero` match the proposal's intent, with `sign_is_negative`/`sign_is_zero` realized as bare `is_negative`/`is_zero` (naming only, no behavioral difference) — see [`exact_sign`'s own type doc](../../module/exact_sign/docs/type/001_sign_classification.md). "No negative zero" holds by construction rather than needing its own check: the backing representation is a two's-complement integer, which has exactly one zero bit pattern — the negative-zero case this hard problem guards against is a float-only artifact, and the family admits no float (→ [hard problem 1](001_float_money_is_wrong.md)).

### Statement

- **Purpose**: zero, positive, and negative are defined; no negative zero.
- **Why**: escrow and debt must not collapse.
- **If missing**: +0 and -0 bugs.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:116-122` | Hard problem 9, verbatim Purpose/Why/If-missing |
