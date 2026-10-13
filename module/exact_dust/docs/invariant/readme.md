# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Full accounting of a split's total across its output shares and leftover.
- **In Scope**: `split_minor`, `slot_minor`, `split_with`, `split_into_with`, and the three `DustTo` destinations.
- **Out of Scope**: Whether a set of outputs sums to zero across a whole ledger (→ `exact_conserve`'s own `docs/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Leftover Is Never Silently Dropped](001_leftover_is_never_silently_dropped.md) | `total == shares + leftover` always holds, whichever `DustTo` destination is chosen | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_dust/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
