# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Totality and mutual exclusivity of the sign classification.
- **In Scope**: `sign_of`, `is_negative`, `is_zero`, `sign_neg_allowed`.
- **Out of Scope**: Where non-negativity is actually enforced (→ `exact_kind`'s own construction-time check) and which direction a saturating clamp picks (→ `exact_add`'s own `invariant/`) — both consume this crate's classification rather than re-deriving it.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Classification Is Total And Mutually Exclusive](001_classification_is_total_and_exclusive.md) | Every `Backing` value classifies into exactly one sign, with no gap and no overlap | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_sign/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
