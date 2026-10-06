# Scene: Reject Extra Digits Parse

### Scope

- **Purpose**: State the smoke lane's sixth step precisely, so its pass condition is checkable without running the (not yet written) lane.
- **Responsibility**: The negative-path parse check for excess fractional digits.
- **In Scope**: The rejected input and expected error variant.
- **Out of Scope**: `exact_parse`'s general design (→ `../crate/009_exact_parse.md`, `../type/009_exact_parse_types.md`).

**Design status**: exercised by step 6 of [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) (`golden`, asserted in `tests/lane_test.rs`'s `the_scenes_land_on_their_golden_values`) — `Decimal::< 2 >::parse("1.234")` is asserted to fail, and the lane prints `extra=1`. The error is renamed: there is no `ParseError` — parsing returns `exact_kind::KindError` directly, and this step's `ExtraDigits` is `KindError::ExcessPrecision` (→ [`exact_parse`](../../module/exact_parse/readme.md), [`type/009_exact_parse_types.md`](../type/009_exact_parse_types.md)).

### Procedure

1. Attempt to parse `"1.234"` at scale 2.
2. Assert it is rejected with `ParseError::ExtraDigits`.

### Pass Criterion

Golden print `extra=1` (a boolean-style flag confirming the rejection fired).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:535` | "Reject "1.234" at scale 2 (ExtraDigits)." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:553,558` | Golden print `extra=1 overflow=1`; "extra=1" |
