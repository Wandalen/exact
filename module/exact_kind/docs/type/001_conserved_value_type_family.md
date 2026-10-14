# Type: Conserved Value Type Family

### Scope

- **Purpose**: Define what `Money`, `Price`, and `Quantity` denote and how the three relate, so a caller reasons about them as a family of fixed-point counts rather than three unrelated wrappers.
- **Responsibility**: `Amount<K, const SCALE: u32>` and its three kinds; `Decimal<const SCALE: u32>`, `Qty<const SCALE: u32>`, `PriceAt<const SCALE: u32>`, its aliases one per kind; and the `Money`/`Quantity`/`Price` aliases built from them.
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

All three are one struct, `Amount<K, SCALE>`: the same one `Minor`, with a
kind `K` — `MoneyKind`, `QtyKind` or `PriceKind` — as a type parameter.
`Decimal<SCALE>` is `Amount<MoneyKind, SCALE>`, `Qty<SCALE>` is
`Amount<QtyKind, SCALE>`, and `PriceAt<SCALE>` is `Amount<PriceKind, SCALE>`.
The kind is held in a zero-sized `PhantomData`, so it costs nothing at run time,
but two kinds are two types and never mix, as two scales do not. `Kind` is
sealed: these three are the only kinds (→
[ADR-006](../decisions/006_one_amount_struct_generic_over_kind.md)).

`Qty<SCALE>` holds the same count as a `Decimal<SCALE>` and adds exactly one property on top:
it cannot hold a negative value (→
[Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md)
for the mechanism). It reuses `Decimal`'s scale mechanism and range checking
rather than re-implementing fixed-point arithmetic a second time.

Three distinct types, each named at `exact_scale::MONEY_SCALE` (6 decimal places) and available at any other scale through its generic form:

| Type | Built as | Notes |
|------|----------|-------|
| `Money` | `Decimal<MONEY_SCALE>` | Signed — a balance going negative is a debt, a state the type must hold |
| `Price` | `PriceAt<MONEY_SCALE>`, `Amount` of the price kind | Signed like `Money` — a price may be a discount; its own type, so a price and an amount of money cannot be mixed (→ [ADR-003](../decisions/003_price_generic_over_scale.md)) |
| `Quantity` | `Qty<MONEY_SCALE>` | Non-negative |

### Construction And Rendering

None of `Decimal`, `Qty` and `PriceAt` has an infallible general constructor. Every
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
| `src/lib.rs:59-65` | `Money`, `Quantity`, `Price` — the three aliases |
| `src/lib.rs:892` | `PriceAt<const SCALE: u32>` — the alias `Amount<PriceKind, SCALE>` |
| `src/lib.rs:357,733,966` | `rescale` on `Decimal`, `Qty` and `PriceAt` — the explicit crossing between scales (→ [ADR-002](../decisions/002_explicit_rescale_between_scales.md)) |
| `src/lib.rs:140-195` | `Kind`, sealed by the private `Sealed`, and the three kinds `MoneyKind`, `QtyKind`, `PriceKind` |
| `src/lib.rs:204-208` | The `Amount<K, const SCALE: u32>` struct — one `minor: Minor` field and a zero-sized `kind: PhantomData<K>` |
| `src/lib.rs:228-237` | `Debug` — prints the kind's own name, `Decimal`, `Qty` or `PriceAt` |
| `src/lib.rs:260` | `Decimal<const SCALE: u32>` — the alias `Amount<MoneyKind, SCALE>` |
| `src/lib.rs:276-297` | `ONE_MINOR`, `ZERO`, `EPSILON`, `MAX`, `MIN` associated constants |
| `src/lib.rs:304-329` | `from_minor`, `from_int` — the two range-checked constructors |
| `src/lib.rs:458-514` | `parse` — the grammar |
| `src/lib.rs:517-530` | `Display` — canonical rendering, trailing zeros trimmed, through `render` |
| `src/lib.rs:546-642` | `render` — the one renderer: trimmed for `{}`, exactly `{:.N}` padded or rounded half to even, then width, fill, alignment, `+` and `0` as for integers (→ [ADR-004](../decisions/004_display_precision_rounds_half_to_even.md)) |
| `src/lib.rs:653` | `Qty<const SCALE: u32>` — the alias `Amount<QtyKind, SCALE>` |
| `src/lib.rs:715-718` | `as_decimal` — the escape hatch back to a signed value |
| `src/lib.rs:985-1078` | `+` and `-` on every kind, four generic impls on `Amount`: each kind's own `checked_add`/`checked_sub` as operators, returning a `Result`; the impls on `Result` let a chain take one `?` (→ [ADR-005](../decisions/005_checked_operators_return_a_result.md), [ADR-006](../decisions/006_one_amount_struct_generic_over_kind.md)) |

### Tests

| File | Relationship |
|------|--------------|
| `tests/parse_render_test.rs` | T01 — round-trip and construction coverage for `Decimal` |
| `tests/checked_arithmetic_test.rs` | T02-T04 — exercises every constructor's range-checking boundary |
| `tests/non_negative_test.rs`'s `leaving_the_type_and_coming_back_passes_through_the_refusal` | `as_decimal` and the `from_decimal` round trip |
