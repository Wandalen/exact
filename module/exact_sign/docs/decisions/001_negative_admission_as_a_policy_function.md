# ADR-001: Negative-Admission As A Policy Function

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

Classifying a value's sign (`sign_of`/`is_negative`/`is_zero`) does not by
itself answer whether a *particular kind* should accept a given value — a
money-like kind admits negative values (a debt), a quantity-like kind does
not. Something needed to express "is this value admissible under this kind's
own policy" at the call site, without scattering a per-kind conditional
(`if Money { true } else { !is_negative(value) }`) wherever a constructor or
an arithmetic result needs the check. `exact_sign` sits at tier 1, below
`exact_kind`, and cannot name a type `exact_kind` declares without inverting
the dependency graph — so the policy had to be expressible without reference
to any specific kind.

## Decision

`sign_neg_allowed(neg_allowed: bool, value: Backing) -> bool` is a free
function taking the policy as an explicit `bool` parameter — `neg_allowed ||
!is_negative(value)` — rather than a per-kind trait method or a bare
constant looked up elsewhere.

## Alternatives Considered

### Option 1: A trait with one method per kind

A `NegativePolicy` trait, implemented once for money-like kinds (always
admits) and once for quantity-like kinds (admits only non-negative values).
Rejected because it requires naming the kinds the trait is implemented for,
which this crate cannot do without depending on `exact_kind` — the reverse of
the family's own dependency order (`exact_kind` depends on `exact_minor` and
`exact_scale`; neither of those, nor `exact_sign`, depends back on
`exact_kind`).

### Option 2: Scattered conditionals at each call site

Check `is_negative(value)` directly wherever a kind's policy needs it,
wrapped in whatever per-kind `if` the call site already has. Rejected per the
function's own doc comment: a scattered conditional reads at the call site as
an accident of which kind happens to be there, not as a stated question about
the value being checked — and it duplicates the same `||` across every call
site that needs it.

## Consequences

**Positive:**
- One function states what "admissible" means, parameterized by the single
  bit that actually varies per kind, kept at a tier below `exact_kind` so no
  dependency has to point backward.
- A caller reads `sign_neg_allowed( kind_allows_negative, candidate )` as a
  question about the value, not as a kind-specific branch it has to get right
  independently at every call site.

**Negative / current state:**
- This function has no real caller yet. `exact_kind::Qty::from_decimal`
  enforces non-negativity directly — `value.minor() < 0` — rather than
  calling into `exact_sign`, because `exact_kind` depends only on
  `exact_minor` and `exact_scale` today (see `exact_kind/Cargo.toml`). This
  crate's own module doc, written ahead of that wiring, describes
  `exact_kind` calling this function "once per kind, at construction" — that
  describes the intended shape, not the current one. The one real cross-crate
  consumer of anything in this crate today is `exact_add::money_saturating_add`,
  and it calls `is_negative` directly (→ [Sign Classification](../type/001_sign_classification.md)), not `sign_neg_allowed`.

**Neutral:**
- `sign_neg_allowed` stays exported and tested on its own terms
  (`tests/sign_classification_test.rs`) regardless of whether `exact_kind`
  has adopted it yet — the function's correctness does not depend on having
  a caller.

## Related

- [Sign Classification](../type/001_sign_classification.md) — `is_negative`, the function this decision is built on
- `exact_kind`'s non-negativity decision — enforces the same property today by a different, more direct route (`Qty::from_decimal`'s own `minor() < 0` check)
