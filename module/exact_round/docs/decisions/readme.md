# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls behind this crate's design, so a later reader finds the reasoning instead of re-deriving or second-guessing it.
- **Responsibility**: Why `HalfEven` departs from the preferred design's planned `Down` default, and why `round_div` lives here rather than in its consumers.
- **In Scope**: Decisions closed by this crate itself.
- **Out of Scope**: Decisions closed in another crate (→ that crate's own `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Half-Even As The Default](001_half_even_as_the_unbiased_default.md) | Superseded — `rounding_default` now returns `Down`, as designed; kept as the record of why `HalfEven` was once the default | 🔄 |
| 002 | [`round_div` Owned By `exact_round`](002_round_div_owned_by_exact_round.md) | Why the shared rounding division lives here rather than in `exact_ratio`/`exact_snap` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_round/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
