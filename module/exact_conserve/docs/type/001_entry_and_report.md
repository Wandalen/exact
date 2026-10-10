# Type: Entry And Report

### Scope

- **Purpose**: Define the input and output shapes of a conservation check, so a producer can build a log this crate accepts without depending on it, and a caller can read a typed-layer failure without re-deriving what it means.
- **Responsibility**: `Entry` (one posting), `Report` (one plain-log audit outcome), and `ConservationError` (why either layer's check could not pass).
- **In Scope**: Field-level representation and the accessors on `Report`; `ConservationError`'s two variants and why each is shaped the way it is.
- **Out of Scope**: How `Report` is computed from a slice of `Entry`, and how the typed layer folds `Money`/`Quantity` legs (→ [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md)).

### Definition

An `Entry` is plain data — an `account: String` (carried for reporting, never for arithmetic), an `asset: String` naming what moved (a currency or an instrument), and a signed `amount_minor: i64`. It carries no invariant of its own and can be constructed by anyone; this crate never interprets the scale the minor units are at, because conservation is a property of the integers and holds at every scale.

A `Report` is the outcome of folding a log: `entries: usize` (how many postings were folded) and `nets: BTreeMap<String, i128>` (each asset's signed sum, keyed by asset — a `BTreeMap`, so the same log always lists its assets in the same order). `is_balanced` is exact equality of every net with zero — no tolerance window. `discrepancy_minor(asset)` returns that asset's signed net directly, and zero for an asset the log never moved: negative means value vanished, positive means it appeared, and those are different investigations. Both are carried forward from `exact_audit`, which summed the whole log into one `net_minor`; one total let a leak in one asset cancel a forgery in another, so the net is now kept per asset (→ [Conservation Is Checked Per Asset](../decisions/002_conservation_checked_per_asset.md)). `Report` is therefore no longer `Copy`, and `is_balanced`/`discrepancy_minor` are no longer `const fn` — both read a map.

`ConservationError` is this crate's own renaming of `exact_audit`'s `AuditError`, per the preferred design — and its shape changes with the rename, not only the name:

- `Overflow` replaces `AuditError::AccumulatorOverflow { at_entry }`. The position-tracking `at_entry` field is dropped, not preserved as a deviation, because the preferred design specifies this crate's error shape explicitly rather than leaving it to be inferred. A caller that needs to bisect a failing log to the entry that broke it can still do so externally; `Overflow` names the condition, not a place to look.
- `NotZero { got: i128 }` is new — the typed layer's `money_sum_assert_zero` needs a way to report a nonzero sum, and the preferred design does not say what type `got` should carry. `i128` matches `Report::discrepancy_minor`'s own type, so a typed-layer failure and a plain-log discrepancy are counted in the same unit.

`ConservationError` is distinct from a `Report` that found a discrepancy: a `ConservationError` means the check itself could not complete, not that it completed with an interesting answer. A `Report` carrying a nonzero net is a successful audit with a discrepancy; `ConservationError::Overflow` is the audit not completing at all, and `ConservationError::NotZero` is the typed layer's own equivalent of that same successful-but-unbalanced outcome.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:94-114` | `Entry` — the plain posting record, its `asset`, and its `new` constructor |
| `src/lib.rs:118-128` | `ConservationError` — `NotZero { got }` and `Overflow` |
| `src/lib.rs:130-140` | `ConservationError`'s `Display` impl — the two outcome messages |
| `src/lib.rs:151-159` | `Report` — `entries` and the per-asset `nets` |
| `src/lib.rs:161-186` | `Report::is_balanced`, `Report::discrepancy_minor` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/conservation_test.rs` | Balance and discrepancy-sign coverage; `the_overflow_error_names_the_representable_range` for `ConservationError::Overflow`'s message; `money_sum_assert_zero_passes_when_legs_cancel_and_reports_the_exact_discrepancy_otherwise` for `NotZero` |
