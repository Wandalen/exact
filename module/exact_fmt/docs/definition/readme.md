# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `FmtError` | enum | `src/lib.rs:31` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |
| `FmtError`'s `Display` impl | trait impl | `src/lib.rs:37` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |
| `FmtError`'s `Error` impl | trait impl | `src/lib.rs:48` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |
| `fmt_into` | fn | `src/lib.rs:81` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |
| `money_fmt` | fn | `src/lib.rs:91` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |
| `qty_fmt` | fn | `src/lib.rs:98` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |
| `price_fmt` | fn | `src/lib.rs:105` | [Decimal Rendering Per Kind](../algorithm/001_decimal_rendering_per_kind.md) |

`Money`, `Price` and `Quantity` appear in these functions' signatures but are declared and documented in `exact_kind`, not here — this crate imports them (`src/lib.rs:27`) without re-exporting them under its own path. `ByteBufWriter` and its `core::fmt::Write` impl (`src/lib.rs:50-69`) are private (no `pub` keyword) and are not part of this crate's public surface, so they are omitted here too — see the algorithm doc for their role.
