# Algorithm: Checked Sub Refuses Below Zero

### Scope

- **Purpose**: State exactly what `Qty::checked_sub` does at and past the zero boundary, so a caller can predict its behavior on any operand pair.
- **Responsibility**: `Qty::<SCALE>::checked_sub`.
- **In Scope**: The below-zero case and its distinction from an overflow or a ceiling breach.
- **Out of Scope**: The general non-negativity invariant this operation is one instance of (→ [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md)); dispatch of this operation from outside this crate (`exact_add::qty_sub` calls this method directly and adds no logic of its own).

### Procedure

1. Delegate to the wrapped `Decimal::checked_sub(self.value, rhs.value)`.
2. If that leaves the backing width or breaches the declared ceiling, propagate `KindError::Overflow`/`ExceedsCeiling` unchanged — a signed `Decimal` subtraction going below zero is not itself a range failure, so neither error fires on that account.
3. Otherwise, route the (possibly negative) `Decimal` result through `Self::from_decimal`, which is the sole point checking `minor() < 0`.
4. A negative result at step 3 returns `KindError::Negative { minor }` — the exact count of minor units the failed subtraction would have produced — never a wrapped, saturated, or zero-clamped value.

### Design Choice: Refuse, Never Clamp

Clamping the result to zero was considered and rejected as part of the wider
non-negativity decision (→
[Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md)):
a clamp is a specific, deliberate falsification — it reports a state (nothing
left) that never actually happened (a negative amount was requested), which
is worse than an error for any caller that later reconciles the result
against a physical count.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:512-524` | `Qty::checked_sub` — delegates to `Decimal::checked_sub` then `Self::from_decimal` |
| `src/lib.rs:68-116` | `KindError` — `Negative` distinguished from `Overflow`/`ExceedsCeiling` |
| `src/lib.rs:428-440` | `Qty::from_decimal` — the choke point `checked_sub` routes through |

### Tests

| File | Relationship |
|------|--------------|
| `tests/non_negative_test.rs`'s `withdrawing_more_than_is_held_is_refused` | Asserts `KindError::Negative { minor : -2_000_000 }` on an over-withdrawal, naming the exact shortfall |
| `tests/non_negative_test.rs`'s `zero_is_reachable_and_one_unit_below_it_is_not` | Emptying to exactly zero succeeds; the next smallest step does not — the pair a clamp-to-zero implementation would get wrong |
| `tests/non_negative_test.rs`'s `a_ceiling_breach_reports_as_a_range_error_and_not_as_a_negative` | Confirms step 2's distinction holds in the other direction too |
