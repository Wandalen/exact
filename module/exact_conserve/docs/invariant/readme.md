# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: The exact-zero conservation guarantee every audit path in this crate enforces.
- **In Scope**: `verify`, `Report::is_balanced`, `money_sum_assert_zero`, `qty_sum_assert_zero`.
- **Out of Scope**: Attribution of a non-zero result to a specific transaction or account, which this crate deliberately does not compute (→ `../algorithm/001_conservation_verification_fold.md`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Sum-To-Zero Is Exact](001_sum_to_zero_is_exact.md) | A balanced log's net is exactly zero, never merely within a tolerance | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_conserve/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
