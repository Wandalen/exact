# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each (the typed doc-definitions above already carry the explanation).
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `DustTo` | enum | `src/lib.rs:65` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |
| `DustError` | enum | `src/lib.rs:78` | [Leftover Correction via Raw Minor-Unit Reconstruction](../decisions/003_leftover_via_raw_minor_reconstruction.md) |
| `DustError`'s `Display` impl | trait impl | `src/lib.rs:88` | [Leftover Correction via Raw Minor-Unit Reconstruction](../decisions/003_leftover_via_raw_minor_reconstruction.md) |
| `money_dust_split` | fn | `src/lib.rs:162` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |
| `money_dust_split_into` | fn | `src/lib.rs:177` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |
| `money_dust_remainder` | fn | `src/lib.rs:203` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |
| `qty_dust_split` | fn | `src/lib.rs:215` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |
| `qty_dust_split_into` | fn | `src/lib.rs:229` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |
| `qty_dust_remainder` | fn | `src/lib.rs:253` | [Equal-Parts Dust Split](../algorithm/001_equal_parts_dust_split.md) |

`impl core::error::Error for DustError {}` (`src/lib.rs:101`) carries no associated item of its own, so it gets no row here — same treatment as every other marker trait impl in this family's `definition/` indexes.

No numbered instance file in this directory — this index is the whole of `definition/` for this crate.
