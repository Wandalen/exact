# Type Doc Definition

### Scope

- **Purpose**: Define the shape and construction guarantees of every type this crate introduces, so a caller can reason about them without reading the implementation.
- **Responsibility**: `Ratio`, the rational multiplier this crate's operations take.
- **In Scope**: Representation and construction of this crate's own types.
- **Out of Scope**: The conserved value types `Ratio` operates on (→ [`exact_kind`](../../../exact_kind/readme.md)); the operations themselves (→ [`algorithm/`](../algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Rational Multiplier](001_rational_multiplier.md) | `Ratio`'s shape, normalization, and construction | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_ratio/docs/type
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
