# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item declared by `exact_arith` or one of the 14 leaf crates this lane consumes — link to that crate's own doc definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `REPEATS` | const | `../../src/lib.rs:45` | — |
| `exact_tenths` | fn | `../../src/lib.rs:55` | [Control Arm Must Disagree](../decisions/001_control_arm_must_disagree.md) |
| `float_tenths` | fn | `../../src/lib.rs:68` | [Control Arm Must Disagree](../decisions/001_control_arm_must_disagree.md) |
| `ledger` | fn | `../../src/lib.rs:91` | [Unchecked Subtraction In Demo Ledger](../pitfall/001_unchecked_subtraction_in_demo_ledger.md) |
| `market_split` | fn | `../../src/lib.rs:132` | — |
| `run` | fn | `../../src/lib.rs:146` | [Control Arm Must Disagree](../decisions/001_control_arm_must_disagree.md), [Library Not Bare Main](../decisions/002_library_not_bare_main.md) |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/smoke_exact_market_split
printf 'pub items in src/lib.rs: '; grep -cE '^pub (const|fn) ' src/lib.rs
printf 'rows in Module Index:    '; grep -cE '^\| `' docs/definition/readme.md
# pub items in src/lib.rs: 6
# rows in Module Index:    6
```
