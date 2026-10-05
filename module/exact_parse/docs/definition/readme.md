# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `money_from_str` | fn | `src/lib.rs:52` | [Decimal Parsing Per Kind](../algorithm/001_decimal_parsing_per_kind.md) |
| `qty_from_str` | fn | `src/lib.rs:62` | [Decimal Parsing Per Kind](../algorithm/001_decimal_parsing_per_kind.md) |
| `price_from_str` | fn | `src/lib.rs:72` | [Decimal Parsing Per Kind](../algorithm/001_decimal_parsing_per_kind.md) |

`KindError`, `Money`, `Price` and `Quantity` appear in these functions' signatures but are declared and documented in `exact_kind`, not here — this crate imports them (`src/lib.rs:38`) without re-exporting them under its own path.
