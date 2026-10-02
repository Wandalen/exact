# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `money_cmp` | fn | `src/lib.rs:24` | [No CmpError, Unconditional Ord](../decisions/001_no_cmp_error_unconditional_ord.md) |
| `qty_cmp` | fn | `src/lib.rs:31` | [No CmpError, Unconditional Ord](../decisions/001_no_cmp_error_unconditional_ord.md) |
| `price_cmp` | fn | `src/lib.rs:38` | [No CmpError, Unconditional Ord](../decisions/001_no_cmp_error_unconditional_ord.md) |
| `money_eq` | fn | `src/lib.rs:45` | [No CmpError, Unconditional Ord](../decisions/001_no_cmp_error_unconditional_ord.md) |
| `price_min` | fn | `src/lib.rs:52` | [No CmpError, Unconditional Ord](../decisions/001_no_cmp_error_unconditional_ord.md) |
| `price_max` | fn | `src/lib.rs:59` | [No CmpError, Unconditional Ord](../decisions/001_no_cmp_error_unconditional_ord.md) |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
