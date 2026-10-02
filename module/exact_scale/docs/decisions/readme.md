# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls this crate made where the preferred design's own listing left a question open.
- **Responsibility**: One ADR instance per distinct decision.
- **In Scope**: This crate's choice to keep scale a compile-time fact rather than building the proposed runtime type.
- **Out of Scope**: Decisions belonging to a sibling crate (e.g. `exact_minor`'s own representation choices) — cite them from there, not here.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Scale Stays Compile-Time, Not Runtime](001_scale_stays_compile_time_not_runtime.md) | Why no runtime `Scale(u8)` type is built, despite the preferred design naming one | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_scale/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
