# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Correctness of the saturating-add clamp direction.
- **In Scope**: `money_saturating_add`, `qty_saturating_add`.
- **Out of Scope**: Totality of the checked arithmetic this crate dispatches to (→ `exact_kind`'s own `invariant/`, which owns that guarantee at the type level).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Saturating Clamp Direction Is Always Correct](001_saturating_clamp_direction_is_always_correct.md) | A failed saturating add always clamps toward the true, unbounded sum's actual direction | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_add/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
