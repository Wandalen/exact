# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: The compile-time-enforced margin between the declared ceiling and the backing width.
- **In Scope**: `CEILING_MINOR_UNITS`, `HEADROOM_FACTOR`, and the assertion linking them.
- **Out of Scope**: `pow10`'s own panic boundary (→ `../non_functional_requirement/001_representable_range_and_headroom.md`), and which scale a conserved value actually uses (→ `exact_kind`'s own `type/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Ceiling Stays Within Headroom](001_ceiling_stays_within_headroom.md) | The declared ceiling can never silently drift within overflow range of `i64::MAX` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_scale/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
