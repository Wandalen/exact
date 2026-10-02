# Type: Entry And Report

### Scope

- **Purpose**: Define the input and output shapes of a conservation check, so a producer can build a log this crate accepts without depending on it, and a caller can read a typed-layer failure without re-deriving what it means.
- **Responsibility**: `Entry` (one posting), `Report` (one plain-log audit outcome), and `ConservationError` (why either layer's check could not pass).
- **In Scope**: Field-level representation and the accessors on `Report`; `ConservationError`'s two variants and why each is shaped the way it is.
- **Out of Scope**: How `Report` is computed from a slice of `Entry`, and how the typed layer folds `Money`/`Quantity` legs (→ [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md)).

### Definition

An `Entry` is plain data — an `account: String` (carried for reporting, never for arithmetic) and a signed `amount_minor: i64`. It carries no invariant of its own and can be constructed by anyone; this crate never interprets the scale the minor units are at, because conservation is a property of the integers and holds at every scale.

A `Report` is the outcome of folding a log: `entries: usize` (how many postings were folded) and `net_minor: i128` (their signed sum). `is_balanced` is exact equality of `net_minor` with zero — no tolerance window. `discrepancy_minor` returns the signed net directly: negative means value vanished, positive means it appeared, and those are different investigations. Both `Entry` and `Report` are carried forward from `exact_audit` with no change in shape or behavior.

`ConservationError` is this crate's own renaming of `exact_audit`'s `AuditError`, per the preferred design — and its shape changes with the rename, not only the name:

- `Overflow` replaces `AuditError::AccumulatorOverflow { at_entry }`. The position-tracking `at_entry` field is dropped, not preserved as a deviation, because the preferred design specifies this crate's error shape explicitly rather than leaving it to be inferred. A caller that needs to bisect a failing log to the entry that broke it can still do so externally; `Overflow` names the condition, not a place to look.
- `NotZero { got: i128 }` is new — the typed layer's `money_sum_assert_zero`/`qty_sum_assert_zero` need a way to report a nonzero sum, and the preferred design does not say what type `got` should carry. `i128` matches `Report::discrepancy_minor`'s own type, and sidesteps a representation problem specific to `Quantity`: a non-negative kind cannot hold a negative leg, so there is no typed signed-quantity value a `got` field could carry for `qty_sum_assert_zero` — a raw minor-unit count is the only representation that works uniformly for both kinds.

`ConservationError` is distinct from a `Report` that found a discrepancy: a `ConservationError` means the check itself could not complete, not that it completed with an interesting answer. A `Report` carrying a nonzero `net_minor` is a successful audit with a discrepancy; `ConservationError::Overflow` is the audit not completing at all, and `ConservationError::NotZero` is the typed layer's own equivalent of that same successful-but-unbalanced outcome.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:87-104` | `Entry` — the plain posting record, and its `new` constructor |
| `src/lib.rs:108-118` | `ConservationError` — `NotZero { got }` and `Overflow` |
| `src/lib.rs:120-130` | `ConservationError`'s `Display` impl — the two outcome messages |
| `src/lib.rs:141-147` | `Report` — `entries` and `net_minor` |
| `src/lib.rs:149-173` | `Report::is_balanced`, `Report::discrepancy_minor` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/conservation_test.rs` | Balance and discrepancy-sign coverage; `the_overflow_error_names_the_representable_range` for `ConservationError::Overflow`'s message; `money_sum_assert_zero_passes_when_legs_cancel_and_reports_the_exact_discrepancy_otherwise` and `qty_sum_assert_zero_passes_only_when_every_leg_is_zero` for `NotZero` |
