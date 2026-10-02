# ADR-002: Library, Not Bare Main

**Date**: 2026-09-30
**Status**: Accepted
**Deciders**: wandalen

## Context

No test suite can execute a bare `src/main.rs` to raise its coverage. While
the whole lane lived in `main.rs`, the only thing that ever ran it was a
person typing `cargo run` — the one file carrying every assertion in the
family's slice was also the one file nothing measured. A project-wide
grading rule already exempts `smoke_*`/`demo_*` crates from the *test* half
of its completeness check, because the lane is its own test — but that
exemption does not help when the coverage tool itself structurally cannot
reach the file at all.

## Decision

Keep the lane's logic in `src/lib.rs` (`exact_tenths`, `float_tenths`,
`ledger`, `market_split`, `run`), leave `src/main.rs` as an argument-free
entry point and nothing else, and drive what moved from `tests/lane_test.rs`.

## Alternatives Considered

### Option 1: Leave everything in `src/main.rs`, rely on the smoke-lane exemption

The exemption already excuses `smoke_*` crates from needing new tests.
Rejected because the exemption addresses not *needing* additional tests, not
whether the *existing* assertions are ever measured by the coverage tool —
those are two different gaps, and this lane had grown well past the size
where leaving the second one unaddressed was acceptable.

### Option 2: Split assertions into a separate test file that duplicates `main.rs`'s logic

Write the same steps twice, once in `main.rs` for `cargo run` and once in a
test file for the suite. Rejected as a straightforward case of the
project's own anti-duplication principle — two copies of the same
walkthrough would drift the moment one was edited without the other.

## Consequences

**Positive:**
- `tests/lane_test.rs` drives the lane from the suite, so every assertion the
  lane makes — now five steps, with the market split added at the Tier 5
  cutover — is measured by ordinary test tooling.
- `cargo run -p smoke_exact_market_split` is unchanged for a human running it
  directly — `main.rs` still calls `smoke_exact_market_split::run()` and
  produces the same output, confirmed current: `src/main.rs` is nine lines, a
  module doc comment plus a `fn main() { smoke_exact_market_split::run(); }`
  body and nothing else.

**Negative:**
- An extra indirection layer (`lib.rs` + thin `main.rs`) for what is
  conceptually a single binary demonstration.

**Neutral:**
- The lane's own assertions and behavior are unchanged by this decision
  itself; only the file and test topology moved, both at the original
  `smoke_exact_arithmetic` split and again, unchanged, at this crate's own
  Tier 5 cutover.

## Related

- [Control Arm Must Disagree](001_control_arm_must_disagree.md) — the assertion this decision makes reachable by a test suite
