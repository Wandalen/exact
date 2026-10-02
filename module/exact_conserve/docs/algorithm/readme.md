# Algorithm Doc Definition

### Scope

- **Purpose**: Collect the procedures this crate implements, so a reader can find how a conservation check actually runs without re-deriving it from source.
- **Responsibility**: One instance per distinct procedure this crate ships.
- **In Scope**: The plain-log fold and the typed per-kind layer built on it.
- **Out of Scope**: The checked arithmetic the typed layer dispatches to (→ `exact_add`'s own crate); the split procedure this audit would catch a failure of (→ [`exact_dust`'s own algorithm/](../../../exact_dust/docs/algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Conservation Verification Fold](001_conservation_verification_fold.md) | Fold a log or a typed slice to a checked total and compare it to zero | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_conserve/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
