# Type Doc Definition

### Scope

- **Purpose**: Define the plain input/output shapes a caller needs in order to use this crate, without depending on it.
- **Responsibility**: One instance per distinct type-level shape this crate declares.
- **In Scope**: `Entry`, `Report`, and `ConservationError`.
- **Out of Scope**: How those shapes are computed or checked (→ [`algorithm/`](../algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Entry And Report](001_entry_and_report.md) | The posting record, the audit outcome, and the error shared by both layers | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_conserve/docs/type
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
