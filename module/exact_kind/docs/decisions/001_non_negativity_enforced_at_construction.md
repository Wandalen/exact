# ADR-001: Non-Negativity As A Separate Type, Enforced At Construction

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

`Money` (`Decimal<MONEY_SCALE>`) is signed: a balance going negative is a
debt, a legitimate state the type must hold. A quantity of a commodity is
not — half a ton of ore short is not a negative holding, it is a failed
withdrawal. The family needed a way to make that distinction enforceable
rather than conventional — and, unlike when this question was first closed
(in the now-deleted `exact_qty`, a crate separate from `exact_decimal`),
`Decimal` and `Qty` are declared in the same crate today, so the question is
no longer only "a separate type or a flag" but also "how does a non-negative
wrapper enforce its refusal when it shares a module, and a representation,
with the signed type it wraps."

A second, narrower question followed from the family's own preferred
fifteen-crate design: that design's `AddError` lists `Overflow`,
`ScaleMismatch`, and `NegNotAllowed` as the error surface for arithmetic.
`exact_kind` instead declares one `KindError` covering construction and
arithmetic together, and it needed its own answer for how a below-zero result
is reported.

## Decision

`Qty<const SCALE: u32>` is a distinct struct wrapping one `Decimal<SCALE>`
field, refusing at construction — and at every arithmetic result — to hold a
negative value. `Decimal` itself stays signed and gains no non-negativity
option. Every path that can produce a `Qty` (`from_decimal`, `from_minor`,
`from_int`, `parse`, `checked_add`, `checked_sub`, `checked_mul_int`) routes
through `from_decimal`, the single choke point checking `value.minor() < 0`.
A violation reports `KindError::Negative { minor }` — the exact count of
minor units the refused value would have carried — distinct from
`KindError::Overflow`/`ExceedsCeiling`, because the arithmetic succeeded and
produced a perfectly representable value that this kind refuses to hold, not
a value the representation itself could not express.

`KindError` carries no `ScaleMismatch` (two different `SCALE` values are two
different Rust types and can never reach a shared function to be compared at
runtime — the compiler refuses it before any check could run) and no
`NegNotAllowed` (the preferred design's name for what `KindError::Negative`
already is). `Negative` is `KindError`'s one addition beyond what the
preferred design's own `AddError` enumerates, chosen specifically to carry
`exact_qty`'s one real refusal forward into the merged type.

## Alternatives Considered

### Option 1: Runtime non-negativity flag on `Decimal`

Add a `non_negative: bool` (or equivalent marker) to `Decimal` itself, checked
by its own arithmetic methods. Rejected because every `Decimal` consumer —
including `Money` and `Price`, which must never carry this restriction —
would have to carry and ignore the flag. Being in the same crate today makes
this worse, not better: `Decimal`'s methods would need to branch on a field
that is meaningless for two of this crate's own three aliases.

### Option 2: Convention only — callers manually check for negative values

Document that quantity-typed callers should check `minor() >= 0` themselves
after each operation. Rejected because it is unenforced: nothing stops a
caller from skipping the check, which is exactly the class of bug a type
system exists to close off.

### Option 3: Report a below-zero result as `KindError::Overflow`

Reuse the existing range-failure variant instead of adding `Negative`.
Rejected because it conflates two different reports: `Overflow` means the
representation could not hold the result at all; a below-zero `Qty` result is
representable (as a `Decimal`) and refused anyway. Collapsing them would send
an investigation into the range budget when the actual event was a withdrawal
that asked for more than was there.

## Consequences

**Positive:**
- A failed withdrawal is a `Result` at the point of subtraction
  (`checked_sub`), where the caller still has the context to act on it (→
  [Checked Sub Refuses Below Zero](../algorithm/001_checked_sub_refuses_below_zero.md)).
- `Qty` reuses `Decimal`'s scale mechanism, backing width, and range checking
  rather than re-implementing fixed-point arithmetic a second time — one
  implementation of scale, one declaration of the backing width, in this
  crate as in the rest of the family.
- `KindError` stays one enum instead of two parallel ones (`Decimal`'s own
  plus a wrapping `AddError`), since every variant it needs — including
  `Negative` — already fits the one type both `Decimal` and `Qty` return.

**Negative:**
- `as_decimal` is a necessary escape hatch for arithmetic that legitimately
  produces a signed intermediate (a price times a quantity, most obviously),
  and getting a `Qty` back out requires passing through `from_decimal` again
  — one extra step a caller must know to take.

**Neutral:**
- `Quantity` is matched to `MONEY_SCALE` rather than a scale chosen
  independently, so a fill-quantity-times-price product lands at a scale the
  backing width already holds.

## Related

- [Conserved Value Type Family](../type/001_conserved_value_type_family.md) — the type family this decision shapes
- [Checked Sub Refuses Below Zero](../algorithm/001_checked_sub_refuses_below_zero.md) — the specific procedure this decision's choke point implements
- [Checked Operations Total](../invariant/002_checked_operations_total.md) — `Negative`'s place in the full error surface
- `exact_sign`'s negative-admission policy decision — classifies the same kind of question (is a value admissible under a kind's policy) at a tier below this one, not yet wired to this crate's own enforcement (→ [`exact_sign`'s own instance](../../../exact_sign/docs/decisions/001_negative_admission_as_a_policy_function.md))
