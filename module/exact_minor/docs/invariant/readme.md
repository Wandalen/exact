# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: The float ban and the totality of every checked (and saturating) operation over the raw backing width.
- **In Scope**: Representation-level and operation-level guarantees of `Backing` itself.
- **Out of Scope**: The declared ceiling, scale, and non-negativity — each layered on by a crate above this one (→ `exact_scale`'s, `exact_kind`'s own `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No Float In Representation](001_no_float_in_representation.md) | No `f32`/`f64` anywhere in this crate's public surface | 🔄 |
| 002 | [Checked Operations Total](002_checked_operations_total.md) | Every checked function returns `Result` and never panics; saturating functions clamp instead | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_minor/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
