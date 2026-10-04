# Algorithm: Decimal Parsing Per Kind

### Scope

- **Purpose**: State exactly what strings `money_from_str`, `qty_from_str` and `price_from_str` accept and what each returns, so a caller can predict their behavior on any input without running it.
- **Responsibility**: The three free functions' dispatch to `exact_kind`'s parser, `qty_from_str`'s added non-negativity refusal, and the compile-time guard keeping this crate's assumed money scale in sync with `exact_scale`.
- **In Scope**: The grammar as inherited through dispatch, the per-kind error surface, and the scale-consistency guard.
- **Out of Scope**: The grammar's own implementation and `Decimal`/`Qty`'s checked operations, owned by `exact_kind::Decimal::parse`/`exact_kind::Qty::parse`; the round-trip's rendering half (→ [`exact_fmt`'s rendering algorithm](../../../exact_fmt/docs/algorithm/001_decimal_rendering_per_kind.md)).

### Grammar

Inherited unchanged from `exact_kind::Decimal::parse` — `Quantity` is `exact_kind::Qty`, itself a thin wrapper over that same `Decimal::parse`: an optional sign (`+` or `-`), at least one integer digit, and an optional fractional part of at most the type's `SCALE` digits after a `.`.

```
decimal := sign? digits ('.' digits)?
sign    := '+' | '-'
digits  := ascii_digit+
```

None of this crate's three functions narrow or widen that grammar — dispatch adds no leniency beyond the one case named next.

### Per-Kind Dispatch

| Function | Dispatches to | Adds |
|----------|---------------|------|
| `money_from_str` | `Money::parse` (`exact_kind::Decimal<MONEY_SCALE>::parse`) | Nothing — direct pass-through |
| `qty_from_str` | `Quantity::parse` (`exact_kind::Qty<MONEY_SCALE>::parse`) | Refuses a negative result with `KindError::Negative` |
| `price_from_str` | `Price::parse` (delegating to `exact_kind::Decimal<MONEY_SCALE>::parse`) | Nothing — a negative price is allowed, like money |

Every input either produces the exact value it spells, or a `KindError`: `Malformed` (grammar violation), `ExcessPrecision` (more fractional digits than the type's scale), `Overflow` (the integer or scaled magnitude leaves the backing width), `ExceedsCeiling` (a value inside the backing width but past the declared ceiling), or — `qty_from_str` only — `Negative` (a well-formed, in-range value that is still below zero). `NaN`, `inf`, exponent forms, and digit separators are all rejected as `Malformed`, same as at the `exact_kind` layer this crate dispatches to.

### Why No `ParseError`

The preferred design for this crate specifies its own `ParseError { Empty, BadChar, ExtraDigits, ScaleTooLarge, Overflow, Sign }`. This crate returns `exact_kind::KindError` directly instead: `Empty`/`BadChar`/`Sign` are all grammar failures `exact_kind` already reports as one `Malformed { reason }`, naming the specific problem in the reason string rather than a separate variant per grammar rule; `ExtraDigits` is `ExcessPrecision`; `Overflow` already exists under that name; and `ScaleTooLarge` is unreachable because `SCALE` is a compile-time const generic — an unrepresentable scale is already a compile error, never a value a running parse call could receive. For the same reason, the preferred design's standalone `parse_reject_extra_digits` guard is not extracted as its own function here: it already lives inside `exact_kind`'s parser, where the digit count is already in scope, and a copy here with no call site besides that same parser would either duplicate the check or force `exact_kind` to depend on this crate — the wrong direction for the family's own dependency graph.

### Cross-Crate Scale Consistency Guard

`src/lib.rs:36` asserts `Money::ONE_MINOR == exact_scale::pow10(exact_scale::MONEY_SCALE)` at compile time. `exact_kind::Money`'s scale and `exact_scale::MONEY_SCALE` are declared in two different crates, connected only by each definition site independently choosing the same numeral; this assertion fails the build the moment the two drift apart, rather than waiting for a parse to silently use the wrong scale.

### Round-Trip Property

`money_from_str`, `qty_from_str` and `price_from_str` are the entry half of the family's round-trip guarantee: `parse(render(v)) == v` and `render(parse(s)) == s` for every representable `v` and canonical spelling `s`. The guarantee itself is proved once, at the `exact_kind` layer both this crate and `exact_fmt` dispatch to (`exact_kind/tests/parse_render_test.rs`) — see [`exact_fmt`'s rendering algorithm doc](../../../exact_fmt/docs/algorithm/001_decimal_rendering_per_kind.md) for the render half and the shared citation.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:1-27` | Module doc — disclosed deviations: no `ParseError`, no standalone `parse_reject_extra_digits` |
| `src/lib.rs:29` | Imports `KindError`, `Money`, `Price`, `Quantity` from `exact_kind` |
| `src/lib.rs:31-36` | The cross-crate scale consistency guard (compile-time assertion) |
| `src/lib.rs:38-46` | `money_from_str` |
| `src/lib.rs:48-56` | `qty_from_str` |
| `src/lib.rs:58-67` | `price_from_str` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/from_str_test.rs` | Per-kind dispatch correctness, `qty_from_str`'s negative refusal, and malformed-text rejection across all three entry points |
| `../exact_kind/tests/parse_render_test.rs` | The grammar and round-trip property this crate dispatches to (owned and proved by `exact_kind`, not re-tested here) |
