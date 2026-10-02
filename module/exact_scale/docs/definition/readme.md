# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `HEADROOM_FACTOR` | const | `src/lib.rs:22` | [Representable Range And Headroom](../non_functional_requirement/001_representable_range_and_headroom.md) |
| `CEILING_WHOLE_UNITS` | const | `src/lib.rs:29` | [Representable Range And Headroom](../non_functional_requirement/001_representable_range_and_headroom.md) |
| `MONEY_SCALE` | const | `src/lib.rs:37` | [Representable Range And Headroom](../non_functional_requirement/001_representable_range_and_headroom.md) |
| `CEILING_MINOR_UNITS` | const | `src/lib.rs:45` | [Representable Range And Headroom](../non_functional_requirement/001_representable_range_and_headroom.md) |
| `pow10` | fn | `src/lib.rs:64` | [Representable Range And Headroom](../non_functional_requirement/001_representable_range_and_headroom.md) |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
