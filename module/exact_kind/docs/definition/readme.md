# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each (the typed doc-definitions above already carry the explanation).
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Money` | type alias | `src/lib.rs:58` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Quantity` | type alias | `src/lib.rs:61` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price` | type alias | `src/lib.rs:64` | [Price Generic Over Scale](../decisions/003_price_generic_over_scale.md) |
| `KindError` | enum | `src/lib.rs:72` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `KindError`'s `Display` impl | trait impl | `src/lib.rs:122` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal<SCALE>` | struct | `src/lib.rs:161` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::ONE_MINOR` | assoc const | `src/lib.rs:180` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::ZERO` | assoc const | `src/lib.rs:183` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::EPSILON` | assoc const | `src/lib.rs:186` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::MAX` | assoc const | `src/lib.rs:193` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::MIN` | assoc const | `src/lib.rs:196` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::from_minor` | fn | `src/lib.rs:203` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::from_int` | fn | `src/lib.rs:219` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::minor` | fn | `src/lib.rs:231` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::rescale` | fn | `src/lib.rs:246` | [Explicit Rescale Between Scales](../decisions/002_explicit_rescale_between_scales.md) |
| `Decimal::whole` | fn | `src/lib.rs:267` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::checked_add` | fn | `src/lib.rs:279` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_sub` | fn | `src/lib.rs:293` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_mul_int` | fn | `src/lib.rs:309` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_neg` | fn | `src/lib.rs:327` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::parse` | fn | `src/lib.rs:347` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal`'s `Display` impl | trait impl | `src/lib.rs:406` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty<SCALE>` | struct | `src/lib.rs:453` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::ZERO` | assoc const | `src/lib.rs:461` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::EPSILON` | assoc const | `src/lib.rs:464` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::MAX` | assoc const | `src/lib.rs:467` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::from_decimal` | fn | `src/lib.rs:474` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::from_minor` | fn | `src/lib.rs:489` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::from_int` | fn | `src/lib.rs:503` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::as_decimal` | fn | `src/lib.rs:518` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::minor` | fn | `src/lib.rs:525` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::rescale` | fn | `src/lib.rs:536` | [Explicit Rescale Between Scales](../decisions/002_explicit_rescale_between_scales.md) |
| `Qty::whole` | fn | `src/lib.rs:544` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::checked_add` | fn | `src/lib.rs:556` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::checked_sub` | fn | `src/lib.rs:570` | [Checked Sub Refuses Below Zero](../algorithm/001_checked_sub_refuses_below_zero.md) |
| `Qty::checked_mul_int` | fn | `src/lib.rs:585` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::parse` | fn | `src/lib.rs:599` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty`'s `Display` impl | trait impl | `src/lib.rs:647` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt<SCALE>` | struct | `src/lib.rs:695` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::ZERO` | assoc const | `src/lib.rs:703` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::MAX` | assoc const | `src/lib.rs:706` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::from_minor` | fn | `src/lib.rs:713` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::minor` | fn | `src/lib.rs:724` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::checked_add` | fn | `src/lib.rs:734` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::checked_sub` | fn | `src/lib.rs:748` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::parse` | fn | `src/lib.rs:762` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::rescale` | fn | `src/lib.rs:772` | [Explicit Rescale Between Scales](../decisions/002_explicit_rescale_between_scales.md) |
| `PriceAt`'s `Display` impl | trait impl | `src/lib.rs:779` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |

`impl core::error::Error for KindError {}` (`src/lib.rs:137`) carries no
associated item of its own, so it gets no row here — same treatment as every
other marker trait impl in this family's `definition/` indexes.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
