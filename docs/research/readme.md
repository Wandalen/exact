# Research Doc Definition

### Scope

- **Purpose**: Hold dated, sourced investigations into questions this family's design raises that aren't settled by reading its own code — starting with whether an existing open-source crate could replace it.
- **Responsibility**: Each instance is one self-contained research question, answered with external, checkable sources.
- **In Scope**: Comparisons against external crates/ecosystems, feasibility questions, and similar investigative work that informs but doesn't itself assert this family's own behavioral contract.
- **Out of Scope**: This family's own behavioral contracts (→ each crate's own `docs/invariant/`, `docs/algorithm/`); design decisions already made (→ each crate's own `docs/decisions/`); the original 15-crate proposal (→ [`../crate/`](../crate/readme.md), [`../hard_problem/`](../hard_problem/readme.md)).

**Design status**: one instance so far, dated 2026-10-01, extended 2026-10-02.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [exact vs. Open-Source Alternatives](001_exact_vs_open_source_alternatives.md) | Whether an existing crate could replace this family or a part of it | ✅ |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/research
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
