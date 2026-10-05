# Pitfall: Unchecked Subtraction In Demo Ledger

### Scope

- **Purpose**: Document a fixed defect in `ledger()`'s seller-posting calculation, per this project's bug-fix documentation convention, so the same shape of mistake — one bounded operand assumed to bound the whole expression — is caught faster next time it appears.
- **Responsibility**: The `seller_minor` computation in `ledger()`.
- **In Scope**: Root cause, why it was not caught earlier, the fix applied, and the generalizable pitfall.
- **Out of Scope**: The auditor's own overflow handling, which is a separate, already-checked path (→ `exact_conserve::verify`, cited directly since that crate has no `docs/algorithm/` instance yet).

### Root Cause

The "seller" posting in `ledger()` computed `amount.minor() - leak_minor` with
a bare `-` on two `i64` values. `amount.minor()` is bounded well inside `i64`
by the exact decimal type's own `CEILING_MINOR_UNITS` headroom, but
`leak_minor` is a public, unvalidated `i64` function parameter with no such
bound — a caller passing `leak_minor` near `i64::MIN` would drive the
subtraction past `i64::MAX`, an overflow.

### Why Not Caught Earlier

Every other arithmetic operation in this crate family already routes through
a `checked_*` method for exactly this reason, so the pattern of "check both
operands" was already established elsewhere — this call site was the one
exception. It went unnoticed because this lane's own `run()` only ever calls
`ledger()` with `leak_minor` equal to `0` or `1`, so the defect never actually
fired in practice; it was a latent gap in the function's general contract, not
a bug the smoke lane's own assertions would ever have surfaced.

### Fix Applied

Replaced the bare `-` with `amount.minor().checked_sub(leak_minor).expect(...)`,
so an out-of-range `leak_minor` now panics explicitly at the call site
instead of silently wrapping into a bogus ledger entry.

### Prevention

A value bounded by one type's own ceiling (here, `Money`/`Backing`) does not
bound an arithmetic expression that mixes it with an unconstrained plain
integer parameter — each operand needs its own check, not just the one that
happens to come from a validated type. When reviewing arithmetic that mixes a
type-bounded value with a raw function parameter, check the raw parameter's
bound independently rather than inheriting confidence from the other operand.

### Pitfall

A value bounded by one type's own ceiling does not bound an arithmetic
expression that mixes it with an unconstrained plain integer — each operand
needs its own check, not just the one that happens to come from a validated
type.

### Carried Forward Intact at the Tier 5 Cutover

This fix, and the regression test guarding it, predate this crate: both were
first written in the retired `smoke_exact_arithmetic`, and both migrated here
unchanged rather than being re-discovered or re-fixed. The fix's own inline
comment keeps its original identifier
(`Fix(smoke_exact_arithmetic_ledger_leak_minor_subtraction_overflow)`) for
traceability back to where this was found, rather than being renamed to match
this crate — confirmed still present verbatim in the current source at the
line range below. No `BUG-NNN` identifier exists for this fix in either the
old or the new crate; it predates this family's current bug-tracking
convention, and the original descriptive identifier is the only one that has
ever existed for it.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:93-115` | `ledger()`'s inline `Fix(smoke_exact_arithmetic_ledger_leak_minor_subtraction_overflow)` comment: root cause, pitfall, and the `checked_sub` fix, in place and unchanged from the predecessor lane |

### Tests

| File | Relationship |
|------|--------------|
| `tests/lane_test.rs:78-120` | `a_leak_minor_that_would_overflow_the_seller_posting_panics_instead_of_wrapping` — the regression test, carried forward intact with its full 5-section fix documentation (Root Cause, Why Not Caught, Fix Applied, Prevention, Pitfall) in its own doc comment; drives `ledger` with `leak_minor: i64::MIN` and asserts the documented panic message, via `#[ should_panic( expected = "..." ) ]` |
| `tests/lane_test.rs` | Also exercises `ledger()` via `run()` at the two realistic `leak_minor` values (`0`, `1`), through `the_lane_runs_end_to_end` and `the_audit_separates_a_balanced_log_from_a_one_unit_leak` — neither reproduces the overflow itself, which requires the unrealistic input only the regression test above supplies |
