# Invariant: Checked Operations Total

### Scope

- **Purpose**: Guarantee that arithmetic on a conserved value never silently loses or invents quantity, so a caller can treat `Err` as the only way range failure ever presents.
- **Responsibility**: Every arithmetic method (`checked_add`, `checked_sub`, `checked_mul_int`, `checked_neg`) and every constructor (`from_minor`, `from_int`, `parse`, `from_decimal`) on `Decimal<SCALE>` and `Qty<SCALE>`.
- **In Scope**: The full error surface — `KindError`'s five variants — and which operations can produce each.
- **Out of Scope**: The raw backing width's own totality, one tier down (→ [`exact_minor`'s instance](../../../exact_minor/docs/invariant/002_checked_operations_total.md)) — this crate implements its own checked arithmetic directly against `Backing`'s own `checked_add`/`checked_sub`/`checked_neg` rather than calling `exact_minor`'s wrapper functions, so this is this crate's own independently-grounded instance of the same property, over a different (and additionally ceiling-constrained) operation surface; the saturating variants, which exist one tier up in `exact_add`, not here.

### Statement

Every arithmetic method and every constructor returns `Result<Self, KindError>`
and never panics on caller input. Failure is always one of five named
conditions, never a silent wrap, truncation, or rounding:

- `Overflow { operation }` — the backing width itself was left (`Backing::MAX`/`MIN`).
- `ExceedsCeiling { minor }` — in width, but past the declared ceiling re-exported from `exact_scale`.
- `ExcessPrecision { digits, scale }` — a parsed string carried more fractional digits than `SCALE` holds.
- `Malformed { reason }` — text outside the parser's grammar.
- `Negative { minor }` — a `Qty` would have gone below zero (→ [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md)).

`Overflow` and `ExceedsCeiling` are deliberately two errors, not one: the
ceiling is a deployment's own declared limit, while the backing width is what
the machine can hold. In practice `Overflow` is unreachable through
`checked_add`/`checked_sub`/`checked_neg` at any operand pair this type can
actually construct — the declared ceiling keeps every representable value far
below `Backing::MIN`/`MAX` — so it is the constructors (`from_int`,
`checked_mul_int`, `parse`) that are the real, exercised `Overflow` paths.

### Rationale

Excess precision is refused rather than repaired for the same reason a float
is refused: truncating a seventh decimal place into a scale-6 type loses it
silently, and rounding invents a value nobody wrote. Distinguishing
`ExceedsCeiling` from `Overflow` means an investigation into a breached
ceiling is not sent chasing a backing-width bug that was never there, and
vice versa.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:68-116` | `KindError` — all five variants |
| `src/lib.rs:199-206` | `Decimal::from_minor` — the `ExceedsCeiling` choke point every other constructor routes through |
| `src/lib.rs:246-301` | `Decimal::checked_add`, `checked_sub`, `checked_mul_int`, `checked_neg` |
| `src/lib.rs:314-370` | `Decimal::parse` — `Malformed`/`ExcessPrecision`/`Overflow` paths |
| `src/lib.rs:503-539` | `Qty::checked_add`, `checked_sub`, `checked_mul_int` — the same contract, plus `Negative` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/checked_arithmetic_test.rs` | T02-T04: exactness, refusal one unit past the ceiling (both signs), refusal at the backing width, and `overflow_is_unreachable_through_add_sub_and_neg_at_the_widest_operands` pinning down the claim above |
| `tests/parse_render_test.rs` | T01: `a_digit_past_the_scale_is_an_error_rather_than_a_rounding`, `float_spellings_and_malformed_text_are_all_refused` |
| `tests/non_negative_test.rs` | T05-T06: `Negative` distinguished from `ExceedsCeiling` |
