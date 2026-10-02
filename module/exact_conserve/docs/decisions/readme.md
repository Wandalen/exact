# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls this crate made where the preferred design's own listing left a question open, including reversals of a decision the crate this one replaces had made.
- **Responsibility**: One ADR instance per distinct decision.
- **In Scope**: This crate's dependency posture, and what became of the `exact_audit` contract it carries forward from.
- **Out of Scope**: Decisions belonging to a sibling crate (e.g. `exact_add`'s own error-type choices) — cite them from there, not here.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Dependency Contract Retired For The Typed Layer](001_dependency_contract_retired_for_typed_layer.md) | Why this crate is no longer zero-dependency, and what of the original contract still holds | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_conserve/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
