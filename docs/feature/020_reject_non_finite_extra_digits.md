# Feature: Reject Non Finite Extra Digits

### Scope

- **Purpose**: Refuse malformed or over-precise numeric strings at the parse boundary instead of silently truncating them.
- **Responsibility**: `exact_parse`'s `parse_reject_extra_digits` check.
- **In Scope**: Rejecting non-finite input and digits beyond the declared scale.
- **Out of Scope**: The happy-path parse itself (→ Feature 005).

**Design status**: implemented in [`exact_parse`](../../module/exact_parse/readme.md), but not as a standalone function — the rejection itself is real (a string like `"1.234"` at scale 2 returns `exact_kind::KindError::ExcessPrecision` via `money_from_str`/`qty_from_str`/`price_from_str`, and non-finite/malformed text returns `KindError::Malformed`), but there is no standalone `parse_reject_extra_digits` call: the guard lives inside `exact_kind`'s own parser, where the digit count is already in scope, rather than being extracted as a free function here — see `exact_parse`'s own [parsing algorithm doc](../../module/exact_parse/docs/algorithm/001_decimal_parsing_per_kind.md) for the full account.

### Statement

A string like `"1.234"` parsed at scale 2 is an `ExtraDigits` error, not a silently truncated `"1.23"`. Rejecting non-finite and over-precise input at the boundary is what keeps "1.10" from reintroducing float parsing further downstream.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:204` | Feature 20 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:535` | The smoke scenario's own "Reject '1.234' at scale 2 (ExtraDigits)" step |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:475-478` | `exact_parse`'s proposed `ParseError` variants |
