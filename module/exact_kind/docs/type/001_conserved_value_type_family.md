# Type: Conserved Value Type Family

### Scope

- **Purpose**: Define what `Money`, `Price`, and `Quantity` denote and how the three relate, so a caller reasons about them as a family of fixed-point counts rather than three unrelated wrappers.
- **Responsibility**: `Decimal<const SCALE: u32>`, `Qty<const SCALE: u32>`, `Price`, and the `Money`/`Quantity` aliases built from them.
- **In Scope**: Representation, construction, rendering, and the type-level scale mechanism.
- **Out of Scope**: The checked-operation contract itself (→ [Checked Operations Total](../invariant/002_checked_operations_total.md)); why non-negativity is a separate wrapper type rather than a flag (→ [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md)); the declared ceiling and `pow10` (→ [`exact_scale`'s own instance](../../../exact_scale/docs/non_functional_requirement/001_representable_range_and_headroom.md)).

### Definition

A `Decimal<SCALE>` is one `exact_minor::Minor` (an `i64`) counting
*minor units*; the value it denotes is `minor × 10⁻ˢᶜᵃˡᵉ`. Adding,
subtracting and negating that count goes through `exact_minor`'s own checked
functions. `SCALE` is a
`const` generic, not a runtime field — `Decimal<6>` and `Decimal<2>` are
different, incompatible Rust types, so mixing scales is a compile error
rather than a check every call site would otherwise have to remember to
perform.

`Qty<SCALE>` wraps one `Decimal<SCALE>` and adds exactly one property on top:
it cannot hold a negative value (→
[Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md)
for the mechanism). It reuses `Decimal`'s scale mechanism and range checking
rather than re-implementing fixed-point arithmetic a second time.

Three distinct types, all at `exact_scale::MONEY_SCALE` (6 decimal places):

| Type | Built as | Notes |
|------|----------|-------|
| `Money` | `Decimal<MONEY_SCALE>` | Signed — a balance going negative is a debt, a state the type must hold |
| `Price` | a struct wrapping one `Money` | Signed like `Money` — a price may be a discount; its own type, so a price and an amount of money cannot be mixed |
| `Quantity` | `Qty<MONEY_SCALE>` | Non-negative |

### Construction And Rendering

None of `Decimal`, `Qty` and `Price` has an infallible general constructor. Every
path — `from_minor`, `from_int`, `parse` (and, for `Qty`, `from_decimal`) —
returns `Result<Self, KindError>`, because an infallible constructor is
exactly a place where a range check did not happen. `ZERO` and `EPSILON` are
the zero-cost exceptions, both representable (and, for `Qty`, non-negative)
at every scale. `parse` accepts an optional sign, at least one integer digit,
and an optional fractional part of at most `SCALE` digits — rejecting
anything wider or outside the grammar rather than rounding or truncating it
(→ [No Float In The Public Constructor Surface](../invariant/001_no_float_in_the_public_constructor_surface.md)).
`Display` renders the exact value with trailing fractional zeros trimmed, so
`"1.50"` parses to the same value as `"1.5"` and both render back as `"1.5"`.

`Qty::as_decimal` is the escape hatch for arithmetic that legitimately leaves
the non-negative type — a price times a quantity, most obviously — handing
back a signed `Decimal` that must pass back through `from_decimal` to become
a `Qty` again.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:57-60` | `Money`, `Quantity` — the two aliases |
| `src/lib.rs:631-635` | The `Price` struct — one `value: Money` field |
| `src/lib.rs:156-160` | The `Decimal<const SCALE: u32>` struct — one `minor: Minor` field |
| `src/lib.rs:176-192` | `ONE_MINOR`, `ZERO`, `EPSILON`, `MAX`, `MIN` associated constants |
| `src/lib.rs:199-223` | `from_minor`, `from_int` — the two range-checked constructors |
| `src/lib.rs:314-370` | `parse` — the grammar |
| `src/lib.rs:373-400` | `Display` — canonical rendering, trailing zeros trimmed |
| `src/lib.rs:412-415` | The `Qty<const SCALE: u32>` struct — one `value: Decimal<SCALE>` field |
| `src/lib.rs:477-480` | `as_decimal` — the escape hatch back to a signed value |

### Tests

| File | Relationship |
|------|--------------|
| `tests/parse_render_test.rs` | T01 — round-trip and construction coverage for `Decimal` |
| `tests/checked_arithmetic_test.rs` | T02-T04 — exercises every constructor's range-checking boundary |
| `tests/non_negative_test.rs`'s `leaving_the_type_and_coming_back_passes_through_the_refusal` | `as_decimal` and the `from_decimal` round trip |
