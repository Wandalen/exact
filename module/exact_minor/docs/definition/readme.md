# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Backing` | type alias | `src/lib.rs:41` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `MinorError` | enum | `src/lib.rs:51` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `MinorError`'s `Display` impl | trait impl | `src/lib.rs:61` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_zero` | fn | `src/lib.rs:76` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `minor_is_zero` | fn | `src/lib.rs:83` | [No Float In Representation](../invariant/001_no_float_in_representation.md) |
| `minor_checked_add` | fn | `src/lib.rs:93` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_checked_sub` | fn | `src/lib.rs:107` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_checked_neg` | fn | `src/lib.rs:122` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_saturating_add` | fn | `src/lib.rs:138` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |
| `minor_saturating_sub` | fn | `src/lib.rs:146` | [Checked Operations Total](../invariant/002_checked_operations_total.md) |

`impl core::error::Error for MinorError {}` (`src/lib.rs:72`) carries no
associated item of its own, so it gets no row here — same treatment as every
other marker trait impl in this family's `definition/` indexes.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
