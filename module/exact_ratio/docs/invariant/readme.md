# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Overflow-safety of the widened multiply shared by every `*_mul_ratio` function.
- **In Scope**: `mul_ratio_minor` and its three per-kind callers.
- **Out of Scope**: `div_round`'s own rounding behavior (→ `exact_round`'s own `docs/`), which this crate dispatches to rather than reimplements.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Widened Product Never Silently Overflows](001_widened_product_never_silently_overflows.md) | The `i128`-widened intermediate product can never wrap or truncate unnoticed | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_ratio/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
