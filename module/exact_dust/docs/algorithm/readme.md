# Algorithm Doc Definition

### Scope

- **Purpose**: Collect the procedures this crate implements, so a reader can find how a value is actually split without re-deriving it from source.
- **Responsibility**: One instance per distinct procedure this crate ships.
- **In Scope**: `exact_dust`'s own split procedure.
- **Out of Scope**: The per-share division primitive itself (`exact_round::round_div`) and the after-the-fact conservation check (→ [`exact_conserve`'s own algorithm/](../../../exact_conserve/docs/algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Equal-Parts Dust Split](001_equal_parts_dust_split.md) | Divide a conserved value into equal integer shares with an explicit remainder destination | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_dust/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
