# Invariant Doc Definition

### Scope

- **Purpose**: Index properties this crate itself must hold, separately from the properties the 14 leaf crates hold about their own types.
- **Responsibility**: One flat list of this crate's own invariant instances.
- **In Scope**: Properties of `exact_arith` as a facade.
- **Out of Scope**: Type-level invariants of any re-exported type (→ the owning leaf crate's own `docs/invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Facade Re-Exports Only](001_facade_re_exports_only.md) | This crate declares no type, function, or constant of its own | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_arith/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
