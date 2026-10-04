# Invariant: No Float In The Public Constructor Surface

### Scope

- **Purpose**: State that no path into or out of `Decimal`/`Qty` ever passes through a float, so a caller can treat "I have one of these types" as proof the value never lost precision through a binary floating-point representation.
- **Responsibility**: Every constructor (`from_minor`, `from_int`, `parse`, `from_decimal`), every accessor (`minor`, `whole`, `as_decimal`), and `Display` for both `Decimal<SCALE>` and `Qty<SCALE>`.
- **In Scope**: This crate's own public surface — the types it exports and the functions that build, inspect, or render them.
- **Out of Scope**: The backing integer itself, which is `exact_minor`'s own leaf-level claim (→ [`exact_minor`'s instance](../../../exact_minor/docs/invariant/001_no_float_in_representation.md)) — this crate re-exports that type (`use exact_minor::Backing;`) rather than restating what it is built from.

### Statement

No float appears in an input or an output position anywhere in this crate's
public surface. `Decimal<SCALE>` stores exactly one `exact_minor::Minor` (an
`i64`), and `Qty<SCALE>` and `Price` each store one wrapped value of it; every constructor takes
an integer (`from_minor`, `from_int`), a string (`parse`), or another value of
the family's own types (`from_decimal`); every accessor returns `Backing` or
`Self`; and `Display` renders by integer division and modulo against
`ONE_MINOR`, never through a float intermediate. The parser's grammar is
narrow by design and explicitly refuses every float spelling it is offered —
`"1e6"`, `"NaN"`, `"inf"`, `"-inf"` — rather than accepting and rounding them.

### Rationale

A float admitted anywhere in this surface would be indistinguishable from an
exact value to every caller downstream — the whole purpose of a fixed-point
type whose scale lives in the type is that *every* representable value is
exact, with no "close enough" value ever constructible. Refusing float
spellings in the parser rather than accepting and rounding them is the same
property applied to text input: a numeric parser that guesses at `"1e6"` is
exactly how a float re-enters a system built to keep it out.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:42-53` | `use exact_minor::{ Backing, Minor, … }` — `Minor`, an `i64`, is the one count ever stored |
| `src/lib.rs:199-223` | `Decimal::from_minor`, `Decimal::from_int` — integer in, `Result<Self, KindError>` out |
| `src/lib.rs:314-370` | `Decimal::parse` — the grammar, with no float spelling accepted |
| `src/lib.rs:373-400` | `Decimal`'s `Display` impl — integer division and modulo, no float intermediate |
| `src/lib.rs:433-469` | `Qty::from_decimal`, `Qty::from_minor`, `Qty::from_int` — the mirrored constructors |
| `src/lib.rs:546-549` | `Qty::parse` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/parse_render_test.rs`'s `float_spellings_and_malformed_text_are_all_refused` | `"NaN"`, `"inf"`, `"-inf"`, `"1e6"`, `"1_000"` are all refused rather than parsed |
| `tests/parse_render_test.rs`'s `every_canonical_spelling_survives_the_round_trip` | Round-tripping holds with no float ever constructed in between |
