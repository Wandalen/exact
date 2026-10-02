# Non-Functional Requirement Doc Definition

### Scope

- **Purpose**: State the numeric budgets this crate's constants must satisfy, checkable by computation rather than asserted by fiat.
- **Responsibility**: The declared ceiling's headroom over the backing width, and `pow10`'s own representable-power boundary.
- **In Scope**: `HEADROOM_FACTOR`, `CEILING_WHOLE_UNITS`, `MONEY_SCALE`, `CEILING_MINOR_UNITS`, `pow10`.
- **Out of Scope**: Which scale or backing width a conserved value actually adopts (→ `exact_kind`'s own `type/`); the backing width itself (→ `exact_minor`'s own `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Representable Range And Headroom](001_representable_range_and_headroom.md) | The declared ceiling stays 1000× below `i64::MAX`, checked at compile time | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_scale/docs/non_functional_requirement
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
