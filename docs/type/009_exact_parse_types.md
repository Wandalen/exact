# Type: exact_parse Types

### Scope

- **Purpose**: Specify `exact_parse`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `from_str` per kind and extra-digit rejection.
- **In Scope**: The functions and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/009_exact_parse.md`).

**Design status**: implemented in [`exact_parse`](../../module/exact_parse/readme.md). `money_from_str`, `qty_from_str`, and `price_from_str` match this proposal's names exactly. Diverges on two points: there is no `ParseError` — every function returns `exact_kind::KindError` directly, since `Empty`/`BadChar`/`Sign` already fold into one `Malformed` variant, `ExtraDigits` is `ExcessPrecision`, and `ScaleTooLarge` is unreachable under this family's compile-time `SCALE`; and there is no standalone `parse_reject_extra_digits` — the guard already lives inside `exact_kind`'s parser. See [`exact_parse`'s algorithm doc](../../module/exact_parse/docs/algorithm/001_decimal_parsing_per_kind.md) for the full account.

### Functions

- `money_from_str`, `qty_from_str`, `price_from_str` — parsing per kind.
- `parse_reject_extra_digits` — the digit-count guard.

### Errors

- `ParseError { Empty, BadChar, ExtraDigits, ScaleTooLarge, Overflow, Sign }`

### Private Detail

Tests for `"1.23"` live here, not in the facade.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:475-478` | `exact_parse`'s full function/error listing |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:509` | "Tests for '1.23' live in exact_parse, not in the facade" |
