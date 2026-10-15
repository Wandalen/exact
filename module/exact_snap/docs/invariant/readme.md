# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: That a `Tick` or `Lot` is never zero-sized.
- **In Scope**: Construction-level guarantees of `Tick` and `Lot`.
- **Out of Scope**: Non-negativity of the underlying price or quantity (→ [`exact_kind`](../../../exact_kind/readme.md)); the rounding division built on top of this guarantee (→ [`exact_round`](../../../exact_round/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Grid Spacing Is Never Zero](001_grid_spacing_never_zero.md) | `Tick::new`/`Lot::new` refuse an exactly-zero spacing | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_snap/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
