# Feature Doc Definition

### Scope

- **Purpose**: Index this crate's own version-scope commitments, separately from any one leaf crate's own feature-level decisions.
- **Responsibility**: One flat list of this crate's feature-scope instances.
- **In Scope**: What `exact_arith` itself commits to as a facade — its re-export completeness and the family-wide exit criteria it closes, not the arithmetic semantics any one re-exported function provides.
- **Out of Scope**: Per-leaf feature scope, where a leaf crate has its own (→ that leaf's own `docs/feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Exact Arith v0.1](001_exact_arith_v0_1.md) | The facade's re-export surface, and which of the family's original exit criteria now hold | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_arith/docs/feature
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
