# Type Doc Definition

### Scope

- **Purpose**: Define the shape and construction guarantees of every type this crate introduces, so a caller can reason about them without reading the implementation.
- **Responsibility**: `Rounding`, the rounding-mode enum, and its stable-name accessor.
- **In Scope**: Representation of this crate's own type.
- **Out of Scope**: Which variant is the family default (→ [`decisions/`](../decisions/readme.md)); the division that applies a chosen mode (→ [`algorithm/`](../algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Rounding Mode](001_rounding_mode.md) | `Rounding`'s eight variants and its stable-name accessor | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_round/docs/type
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
