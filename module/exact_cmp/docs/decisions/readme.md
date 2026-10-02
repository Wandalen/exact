# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls behind this crate's design, so a later reader finds the reasoning instead of re-deriving or second-guessing it.
- **Responsibility**: Why this crate defines no `CmpError` and no conditional `Ord`.
- **In Scope**: Decisions closed by this crate itself.
- **Out of Scope**: The same unreachable-variant reasoning as applied to arithmetic (→ [`exact_ratio`'s decisions](../../../exact_ratio/docs/decisions/readme.md)) or to the conserved value types themselves (→ [`exact_kind`](../../../exact_kind/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No CmpError, Unconditional Ord](001_no_cmp_error_unconditional_ord.md) | Why comparison here is infallible rather than guarded by a scale check | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_cmp/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
