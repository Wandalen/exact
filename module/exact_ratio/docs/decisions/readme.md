# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls behind this crate's design, so a later reader finds the reasoning instead of re-deriving or second-guessing it.
- **Responsibility**: Why `RatioError`'s shape departs from the preferred design's own listing.
- **In Scope**: Decisions closed by this crate itself.
- **Out of Scope**: The same unreachable-variant reasoning as applied to comparison (→ [`exact_cmp`'s decisions](../../../exact_cmp/docs/decisions/readme.md)) or to the conserved value types themselves (→ [`exact_kind`](../../../exact_kind/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Ratio Error Without Scale Mismatch Or Bad Rounding](001_ratio_error_without_scale_mismatch_or_bad_rounding.md) | Why `RatioError` drops two preferred-design variants and adds `Negative` (`Inexact` is recorded in `exact_round`'s ADR-004) | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_ratio/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
