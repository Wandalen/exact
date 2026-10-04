# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each (the typed doc-definitions above already carry the explanation).
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Money` | type alias | `src/lib.rs:57` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Quantity` | type alias | `src/lib.rs:60` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `KindError` | enum | `src/lib.rs:68` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `KindError`'s `Display` impl | trait impl | `src/lib.rs:118` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal<SCALE>` | struct | `src/lib.rs:157` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::ONE_MINOR` | assoc const | `src/lib.rs:176` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::ZERO` | assoc const | `src/lib.rs:179` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::EPSILON` | assoc const | `src/lib.rs:182` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::MAX` | assoc const | `src/lib.rs:189` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::MIN` | assoc const | `src/lib.rs:192` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::from_minor` | fn | `src/lib.rs:199` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::from_int` | fn | `src/lib.rs:215` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::minor` | fn | `src/lib.rs:227` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::whole` | fn | `src/lib.rs:234` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::checked_add` | fn | `src/lib.rs:246` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_sub` | fn | `src/lib.rs:260` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_mul_int` | fn | `src/lib.rs:276` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_neg` | fn | `src/lib.rs:294` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::parse` | fn | `src/lib.rs:314` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal`'s `Display` impl | trait impl | `src/lib.rs:373` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty<SCALE>` | struct | `src/lib.rs:412` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::ZERO` | assoc const | `src/lib.rs:420` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::EPSILON` | assoc const | `src/lib.rs:423` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::MAX` | assoc const | `src/lib.rs:426` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::from_decimal` | fn | `src/lib.rs:433` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::from_minor` | fn | `src/lib.rs:448` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::from_int` | fn | `src/lib.rs:462` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::as_decimal` | fn | `src/lib.rs:477` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::minor` | fn | `src/lib.rs:484` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::whole` | fn | `src/lib.rs:491` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::checked_add` | fn | `src/lib.rs:503` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::checked_sub` | fn | `src/lib.rs:517` | [Checked Sub Refuses Below Zero](../algorithm/001_checked_sub_refuses_below_zero.md) |
| `Qty::checked_mul_int` | fn | `src/lib.rs:532` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::parse` | fn | `src/lib.rs:546` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty`'s `Display` impl | trait impl | `src/lib.rs:594` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price` | struct | `src/lib.rs:632` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::ZERO` | assoc const | `src/lib.rs:640` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::MAX` | assoc const | `src/lib.rs:643` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::from_minor` | fn | `src/lib.rs:650` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::minor` | fn | `src/lib.rs:661` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::checked_add` | fn | `src/lib.rs:671` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::checked_sub` | fn | `src/lib.rs:685` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price::parse` | fn | `src/lib.rs:699` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price`'s `Display` impl | trait impl | `src/lib.rs:706` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |

`impl core::error::Error for KindError {}` (`src/lib.rs:133`) carries no
associated item of its own, so it gets no row here — same treatment as every
other marker trait impl in this family's `definition/` indexes.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
