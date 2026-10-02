# Algorithm Doc Definition

### Scope

- **Purpose**: State exactly how this crate's operations compute their result, so a caller can predict the output — and the failure mode — of a call without reading the implementation.
- **Responsibility**: `round_div` — the sign-normalized, mode-driven rounding division both `exact_ratio` and `exact_snap` share.
- **In Scope**: The procedure's steps and its correctness argument, including `RoundError`.
- **Out of Scope**: Which mode applies by default (→ [`decisions/`](../decisions/readme.md)); the meaning of each `Rounding` variant itself (→ [`type/`](../type/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Rounding Division](001_rounding_division.md) | Sign normalization, truncating divide, and mode-driven remainder adjustment | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_round/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
