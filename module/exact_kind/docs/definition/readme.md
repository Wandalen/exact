# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each (the typed doc-definitions above already carry the explanation).
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Money` | type alias | `src/lib.rs:59` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Quantity` | type alias | `src/lib.rs:62` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Price` | type alias | `src/lib.rs:65` | [Price Generic Over Scale](../decisions/003_price_generic_over_scale.md) |
| `KindError` | enum | `src/lib.rs:73` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `KindError`'s `Display` impl | trait impl | `src/lib.rs:123` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Kind` | trait | `src/lib.rs:179` | [One `Amount` Struct, Generic Over The Kind](../decisions/006_one_amount_struct_generic_over_kind.md) |
| `MoneyKind` | struct | `src/lib.rs:183` | [One `Amount` Struct, Generic Over The Kind](../decisions/006_one_amount_struct_generic_over_kind.md) |
| `QtyKind` | struct | `src/lib.rs:187` | [One `Amount` Struct, Generic Over The Kind](../decisions/006_one_amount_struct_generic_over_kind.md) |
| `PriceKind` | struct | `src/lib.rs:191` | [One `Amount` Struct, Generic Over The Kind](../decisions/006_one_amount_struct_generic_over_kind.md) |
| `Amount<K, SCALE>` | struct | `src/lib.rs:204` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Amount`'s `Debug` impl | trait impl | `src/lib.rs:231` | [One `Amount` Struct, Generic Over The Kind](../decisions/006_one_amount_struct_generic_over_kind.md) |
| `Decimal<SCALE>` | type alias | `src/lib.rs:260` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::ONE_MINOR` | assoc const | `src/lib.rs:276` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::ZERO` | assoc const | `src/lib.rs:284` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::EPSILON` | assoc const | `src/lib.rs:287` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::MAX` | assoc const | `src/lib.rs:294` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::MIN` | assoc const | `src/lib.rs:297` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::from_minor` | fn | `src/lib.rs:304` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::from_int` | fn | `src/lib.rs:321` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::minor` | fn | `src/lib.rs:333` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::rescale` | fn | `src/lib.rs:357` | [Explicit Rescale Between Scales](../decisions/002_explicit_rescale_between_scales.md) |
| `Decimal::whole` | fn | `src/lib.rs:378` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal::checked_add` | fn | `src/lib.rs:390` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_sub` | fn | `src/lib.rs:404` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_mul_int` | fn | `src/lib.rs:420` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::checked_neg` | fn | `src/lib.rs:438` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `Decimal::parse` | fn | `src/lib.rs:458` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Decimal`'s `Display` impl | trait impl | `src/lib.rs:517` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty<SCALE>` | type alias | `src/lib.rs:653` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::ZERO` | assoc const | `src/lib.rs:658` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::EPSILON` | assoc const | `src/lib.rs:661` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::MAX` | assoc const | `src/lib.rs:664` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::from_decimal` | fn | `src/lib.rs:671` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::from_minor` | fn | `src/lib.rs:686` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::from_int` | fn | `src/lib.rs:700` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::as_decimal` | fn | `src/lib.rs:715` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::minor` | fn | `src/lib.rs:722` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::rescale` | fn | `src/lib.rs:733` | [Explicit Rescale Between Scales](../decisions/002_explicit_rescale_between_scales.md) |
| `Qty::whole` | fn | `src/lib.rs:741` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Qty::checked_add` | fn | `src/lib.rs:753` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::checked_sub` | fn | `src/lib.rs:767` | [Checked Sub Refuses Below Zero](../algorithm/001_checked_sub_refuses_below_zero.md) |
| `Qty::checked_mul_int` | fn | `src/lib.rs:782` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty::parse` | fn | `src/lib.rs:796` | [Non-Negativity Enforced At Construction](../decisions/001_non_negativity_enforced_at_construction.md) |
| `Qty`'s `Display` impl | trait impl | `src/lib.rs:844` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt<SCALE>` | type alias | `src/lib.rs:892` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::ZERO` | assoc const | `src/lib.rs:897` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::MAX` | assoc const | `src/lib.rs:900` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::from_minor` | fn | `src/lib.rs:907` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::minor` | fn | `src/lib.rs:918` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::checked_add` | fn | `src/lib.rs:928` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::checked_sub` | fn | `src/lib.rs:942` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::parse` | fn | `src/lib.rs:956` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `PriceAt::rescale` | fn | `src/lib.rs:966` | [Explicit Rescale Between Scales](../decisions/002_explicit_rescale_between_scales.md) |
| `PriceAt`'s `Display` impl | trait impl | `src/lib.rs:973` | [Conserved Value Type Family](../type/001_conserved_value_type_family.md) |
| `Add` for `Amount` — `a + b` on every kind | trait impl | `src/lib.rs:1037` | [`+` And `-` Return A `Result`](../decisions/005_checked_operators_return_a_result.md) |
| `Add<Amount>` for `Result<Amount, KindError>` — `( a + b ) + c` | trait impl | `src/lib.rs:1048` | [`+` And `-` Return A `Result`](../decisions/005_checked_operators_return_a_result.md) |
| `Sub` for `Amount` — `a - b` on every kind | trait impl | `src/lib.rs:1059` | [`+` And `-` Return A `Result`](../decisions/005_checked_operators_return_a_result.md) |
| `Sub<Amount>` for `Result<Amount, KindError>` — `( a + b ) - c` | trait impl | `src/lib.rs:1070` | [`+` And `-` Return A `Result`](../decisions/005_checked_operators_return_a_result.md) |

`impl core::error::Error for KindError {}` (`src/lib.rs:138`) carries no
associated item of its own, so it gets no row here — same treatment as every
other marker trait impl in this family's `definition/` indexes. So do
`impl Kind for MoneyKind {}`, `QtyKind` and `PriceKind` (`src/lib.rs:193-195`).
The private `sealed` module, its `Sealed` trait and impls, and `Amount`'s
private `value()`/`from_value()` are not public, so they are not indexed.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
