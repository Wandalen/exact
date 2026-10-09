# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Rounding` | enum | `src/lib.rs:38` | [Rounding Mode](../type/001_rounding_mode.md) |
| `rounding_default` | fn | `src/lib.rs:83` | [Half-Even As The Default](../decisions/001_half_even_as_the_unbiased_default.md) |
| `rounding_name` | fn | `src/lib.rs:93` | [Rounding Mode](../type/001_rounding_mode.md) |
| `RoundError` | enum | `src/lib.rs:110` | [Rounding Division](../algorithm/001_rounding_division.md) |
| `RoundError`'s `Display` impl | trait impl | `src/lib.rs:121` | [Rounding Division](../algorithm/001_rounding_division.md) |
| `RoundError`'s `Error` impl | trait impl | `src/lib.rs:134` | [Rounding Division](../algorithm/001_rounding_division.md) |
| `round_div` | fn | `src/lib.rs:147` | [Rounding Division](../algorithm/001_rounding_division.md) |
| `round_div_wide` | fn | `src/lib.rs:171` | [Rounding Division](../algorithm/001_rounding_division.md) |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
