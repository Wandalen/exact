# ADR-001: Control Arm Must Disagree

**Date**: 2026-09-30
**Status**: Accepted
**Deciders**: wandalen

## Context

A lane that only runs the exact path proves the exact path does not crash. It
does not prove the path is exact — a lane built entirely on `f64` would print
the same cheerful verdict for nine of this lane's ten steps. The lane needed
a way to actually discriminate between exact and inexact arithmetic, not just
exercise the exact path and hope.

## Decision

Every central claim is made twice — once through the exact types
(`exact_tenths`) and once through `f64` (`float_tenths`) — and the lane
asserts that the two arms **disagree** (`control != 1.0_f64`). This is
inherited from plan 011's rule, not invented locally.

## Alternatives Considered

### Option 1: Run the exact path only

Simplest option; drop the `f64` arm entirely. Rejected because it cannot
distinguish "the exact arithmetic is correct" from "nothing in this lane
would have caught it if it weren't" — the two are indistinguishable from the
exact-only lane's output alone.

### Option 2: Run both arms, assert they agree within a tolerance

Run both and treat close-enough agreement as success. Rejected because this
is precisely the failure mode the family's conservation work exists to make
impossible: a tolerance is how a check comes to pass the one error small
enough to be worth hiding. Requiring disagreement, not agreement, is what
makes the control arm's own correctness self-evident from the lane's output.

## Consequences

**Positive:**
- Already exercised against the predecessor lane (`smoke_exact_arithmetic`):
  a mutation probe (task 156) replaced `float_tenths` with a literal `1.0`,
  and the lane failed, as it must — confirming the assertion actually
  discriminates. The assertion and its surrounding steps carried forward
  unchanged into this crate at the Tier 5 cutover, so the same discrimination
  still holds here (`run()`'s own `assert!( control != 1.0_f64, ... )` and
  `tests/lane_test.rs`'s `the_control_arm_is_still_wrong`).
- If a future change made the exact path inexact, the arms would agree, the
  assertion would fail, and the lane would go red — a regression in exactness
  cannot silently pass.

**Negative:**
- `f64` now appears in the codebase at all, in exactly this one crate — a
  type the family otherwise bans for conserved values
  (→ [`exact_arith`'s own feature doc](../../../exact_arith/docs/feature/001_exact_arith_v0_1.md)
  names this crate's control arm as the family's one deliberate exception).

**Neutral:**
- The lane's job is partly to be wrong on the control side; a lane where the
  control arm stops being wrong has stopped discriminating, and that failure
  mode (a silently-passing lane) is reported as a hard failure, not a pass.

## Related

- [Library Not Bare Main](002_library_not_bare_main.md) — the placement decision that lets a test suite actually run this assertion
- [`exact_kind`: No Float In The Public Constructor Surface](../../../exact_kind/docs/invariant/001_no_float_in_the_public_constructor_surface.md) — the invariant this lane's control arm exists to demonstrate the necessity of (the successor, at the public-surface level this lane actually calls, to the predecessor family's `exact_decimal/docs/invariant/001_no_float_in_representation.md`)
