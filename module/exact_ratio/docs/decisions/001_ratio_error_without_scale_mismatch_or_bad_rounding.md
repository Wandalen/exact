# ADR-001: Ratio Error Without Scale Mismatch Or Bad Rounding

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

The preferred design names `RatioError { DivZero, Overflow, ScaleMismatch,
BadRounding }`. Under this family's representation, two of those four
variants have no call site that could ever construct them, and the crate
had to decide whether to keep them as inert markers or drop them.

## Decision

`RatioError` carries three variants — `DivZero`, `Overflow`, and `Negative {
minor }` — dropping `ScaleMismatch` and `BadRounding` entirely and adding
`Negative` in their place.

- **`ScaleMismatch` is unreachable.** It exists in the preferred design for a
  representation where scale is a runtime value two operands could disagree
  on. Under this family's `Decimal<const SCALE: u32>`, `Decimal<6>` and
  `Decimal<9>` are different Rust types — the same reasoning
  [`exact_cmp`'s own deviation](../../../exact_cmp/docs/decisions/001_no_cmp_error_unconditional_ord.md)
  already rests on. A mismatch is refused by the compiler before any
  function in this crate runs, so there is no runtime path that could ever
  construct this variant.
- **`BadRounding` is unreachable.** `exact_round::Rounding` is a closed
  three-variant enum; every value of it is already a valid rounding mode.
  There is no way to construct an invalid one through this crate's public
  API for the variant to report.
- **`Negative` is the one real failure the preferred design's listing
  missed.** A `Qty`-kind multiply or divide can legitimately produce a
  result below zero — a negative-numerator ratio applied to a quantity, for
  instance — and that failure needed a variant to report it, which neither
  `ScaleMismatch` nor `BadRounding` could stand in for.

## Alternatives Considered

### Option 1: Keep `ScaleMismatch` and `BadRounding` as dead variants

Declare all four preferred-design variants and simply never construct the
two unreachable ones. Rejected: an error variant with no reachable
construction site is dead code wearing a doc comment, and keeping it invites
a caller to write a `match` arm — or a test — for a case that can never
occur, which is cost paid forever for a possibility already closed at
compile time.

### Option 2: A single catch-all `Invalid` variant instead of per-case ones

Collapse every failure into one opaque variant. Rejected: the family's own
general principle, already stated in `exact_kind`'s module doc, is that a
generic "invalid" error moves the investigation to the debugger. `DivZero`,
`Overflow`, and `Negative` each point at a different fix.

## Consequences

**Positive:**
- Every variant `RatioError` declares is reachable through this crate's
  public API, so a caller's exhaustive `match` has no dead arm to maintain.
- The real, previously-unlisted failure (`Negative`) is reported precisely
  instead of folding into `Overflow`, which would have sent an investigation
  looking for a range breach that never happened.

**Negative:**
- A caller porting code written against the preferred design's four-variant
  listing must drop two match arms rather than finding them merely unused.

**Neutral:**
- The same two-variant drop, for the same reason, already applies to
  `exact_kind::KindError` (no `ScaleMismatch`) and `exact_cmp::CmpError`
  (dropped entirely) — this is the third instance of one judgment call
  across the family, not an independent one.

## Related

- [Rational Multiplier](../type/001_rational_multiplier.md) — the type this error reports failures for
- [Widened Multiply Before Narrow](../algorithm/001_widened_multiply_before_narrow.md) — the operation `RatioError::Overflow` and `RatioError::Negative` are actually returned from
- [`exact_cmp`'s own deviation](../../../exact_cmp/docs/decisions/001_no_cmp_error_unconditional_ord.md) — the same compile-time-scale reasoning applied to comparison instead of arithmetic
