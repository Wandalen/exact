# Algorithm Doc Definition

### Scope

- **Purpose**: Specify the exact step-by-step behavior of this crate's one genuinely procedural operation, so a caller can predict it at every boundary rather than from the happy path alone.
- **Responsibility**: `Qty::checked_sub`'s below-zero procedure.
- **In Scope**: Procedures with a real branch structure worth walking through step by step.
- **Out of Scope**: Construction and rendering, which are documented as shape rather than procedure (→ `type/`); parsing's own grammar, which stays in this crate's own doc comments and tests rather than a dedicated instance here — see this crate's own `docs/readme.md` for why.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Checked Sub Refuses Below Zero](001_checked_sub_refuses_below_zero.md) | `Qty::checked_sub`'s exact behavior at and past the zero boundary | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_kind/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
