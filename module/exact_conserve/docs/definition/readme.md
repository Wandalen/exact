# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each (the typed doc-definitions above already carry the explanation).
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Entry` | struct | `src/lib.rs:95` | [Entry And Report](../type/001_entry_and_report.md) |
| `Entry::new` | fn | `src/lib.rs:111` | [Entry And Report](../type/001_entry_and_report.md) |
| `ConservationError` | enum | `src/lib.rs:119` | [Entry And Report](../type/001_entry_and_report.md) |
| `ConservationError`'s `Display` impl | trait impl | `src/lib.rs:131` | [Entry And Report](../type/001_entry_and_report.md) |
| `Report` | struct | `src/lib.rs:152` | [Entry And Report](../type/001_entry_and_report.md) |
| `Report::is_balanced` | fn | `src/lib.rs:172` | [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) |
| `Report::discrepancy_minor` | fn | `src/lib.rs:184` | [Entry And Report](../type/001_entry_and_report.md) |
| `Report`'s `Display` impl | trait impl | `src/lib.rs:193` | [Entry And Report](../type/001_entry_and_report.md) |
| `verify` | fn | `src/lib.rs:230` | [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) |
| `money_conserve_into` | fn | `src/lib.rs:251` | [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) |
| `qty_conserve_into` | fn | `src/lib.rs:262` | [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) |
| `money_sum_assert_zero` | fn | `src/lib.rs:273` | [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) |

`impl core::error::Error for ConservationError {}` (`src/lib.rs:143`) carries no associated item of its own, so it gets no row here — same treatment as every other marker trait impl in this family's `definition/` indexes.

No numbered instance file in this directory — this index is the whole of `definition/` for this crate.
