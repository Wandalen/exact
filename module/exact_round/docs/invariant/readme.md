# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Determinism and bounded correction of `round_div`'s result.
- **In Scope**: `round_div`'s arithmetic types and its distance from the true quotient.
- **Out of Scope**: Why `HalfEven` is the unbiased default (→ `../decisions/`), and the tie-breaking procedure itself (→ `../algorithm/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Rounding Is Integer-Only And Bounded](001_rounding_is_integer_only_and_bounded.md) | No float ever appears, and the result never diverges from the true quotient by more than one unit | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_round/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
