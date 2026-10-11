# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls behind this crate's design, so a later reader finds the reasoning instead of re-deriving or second-guessing it.
- **Responsibility**: Decisions that span every crate this facade re-exports.
- **In Scope**: A change made the same way across the family's crates, recorded once here rather than once per crate.
- **Out of Scope**: A decision closed by one crate alone (→ that crate's own `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Every Operation Is Generic Over The Scale](001_operations_generic_over_scale.md) | Why the 46 functions over a conserved value take `< const S : u32 >`, and which callers must now name the scale | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_arith/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
