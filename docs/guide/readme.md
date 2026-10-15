# Guide Doc Definition

### Scope

- **Purpose**: Give a newcomer the real, current shape of this crate family — what each crate does and how they depend on each other — in one place, fast.
- **Responsibility**: Onboarding instances covering the 16 crates as actually implemented, kept current against `module/`.
- **In Scope**: Per-crate purpose, the real dependency graph, suggested reading order.
- **Out of Scope**: The original 15-crate proposal and its own dependency tree (→ [`../readme.md`](../readme.md) § Dependency Tree, [`../crate/`](../crate/readme.md) — historical, proposal-only); per-crate struct/enum/function surface (→ [`../type/`](../type/readme.md)); per-crate design rationale and decisions (→ each crate's own `docs/decisions/`).

**Design status**: implemented — this collection describes the 16 crates as they exist in `module/` today, not a proposal.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Crate Family Overview](001_crate_family_overview.md) | Purpose of each of the 16 crates, the real dependency tree, and a fast reading order | ✅ |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/guide
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
