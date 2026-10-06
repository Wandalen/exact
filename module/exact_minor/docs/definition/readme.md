# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Backing` | type alias | `src/lib.rs:39` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `Minor` | struct | `src/lib.rs:51` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `minor_from_i64` | fn | `src/lib.rs:55` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `minor_to_i64` | fn | `src/lib.rs:62` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `MinorError` | enum | `src/lib.rs:206` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `MinorError`'s `Display` impl | trait impl | `src/lib.rs:222` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_zero` | fn | `src/lib.rs:238` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `minor_is_zero` | fn | `src/lib.rs:245` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `minor_checked_add` | fn | `src/lib.rs:256` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_checked_sub` | fn | `src/lib.rs:272` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_checked_neg` | fn | `src/lib.rs:288` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_saturating_add` | fn | `src/lib.rs:304` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_saturating_sub` | fn | `src/lib.rs:312` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |

Only with `--features i128`:

| Item | Kind | Declared |
|------|------|----------|
| `MinorWide` | struct | `src/lib.rs:75` |
| `minor_wide_from_i128` | fn | `src/lib.rs:80` |
| `minor_wide_to_i128` | fn | `src/lib.rs:88` |
| `From< Minor > for MinorWide` | trait impl | `src/lib.rs:94` |
| `TryFrom< MinorWide > for Minor` | trait impl | `src/lib.rs:104` |
| `minor_wide_zero` | fn | `src/lib.rs:122` |
| `minor_wide_is_zero` | fn | `src/lib.rs:130` |
| `minor_wide_checked_add` | fn | `src/lib.rs:142` |
| `minor_wide_checked_sub` | fn | `src/lib.rs:159` |
| `minor_wide_checked_neg` | fn | `src/lib.rs:175` |
| `minor_wide_saturating_add` | fn | `src/lib.rs:187` |
| `minor_wide_saturating_sub` | fn | `src/lib.rs:195` |

`impl core::error::Error for MinorError {}` (`src/lib.rs:234`) carries no
associated item of its own, so it gets no row here — same treatment as every
other marker trait impl in this family's `definition/` indexes.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
