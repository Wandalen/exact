# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls behind this crate's design, so a later reader finds the reasoning instead of re-deriving or second-guessing it.
- **Responsibility**: Why `WireError` carries two variants beyond the preferred design's own three.
- **In Scope**: Decisions closed by this crate itself.
- **Out of Scope**: The wire layout those variants guard (→ [`format/`](../format/readme.md)); the same addition made for the same reason in a sibling crate (→ [`exact_ratio`'s decisions](../../../exact_ratio/docs/decisions/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Wire Error Adds Overflow And Negative](001_wire_error_overflow_and_negative_variants.md) | Why decoding needs two more failure variants than the preferred design named | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_bytes/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
