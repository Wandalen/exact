# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls this crate made where the preferred design's own listing left a question open.
- **Responsibility**: One ADR instance per distinct decision.
- **In Scope**: Dependency choice, split-function surface, and leftover-correction mechanism for this crate.
- **Out of Scope**: Decisions belonging to a sibling crate (e.g. `exact_round`'s own rounding-mode design, or `exact_snap`'s own `Overflow`-folding precedent) — cite them from there, not here.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Direct `exact_round` Dependency, Not `exact_ratio`](001_direct_exact_round_dependency.md) | Why this crate depends on `exact_round` directly instead of through `exact_ratio` | 🔄 |
| 002 | [Equal-Count Split Surface](002_equal_count_split_surface.md) | Why `parts` is a plain count, and why there is no `price_dust_split` | 🔄 |
| 003 | [Leftover Correction via Raw Minor-Unit Reconstruction](003_leftover_via_raw_minor_reconstruction.md) | Why the `First` correction happens at the raw-minor stage, and folds into `Overflow` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_dust/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              3
# rows in Overview Table: 3
```
