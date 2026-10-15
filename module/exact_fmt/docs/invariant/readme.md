# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Bounds-safety of the allocation-free buffer-writing primitive.
- **In Scope**: `fmt_into`, `ByteBufWriter::write_str`.
- **Out of Scope**: What exactly ends up in `buf` after a failed multi-piece write (→ `../algorithm/`, which documents that case in full).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Buffer Writes Stay In Bounds](001_buffer_writes_stay_in_bounds.md) | `fmt_into` never writes past the end of a caller-supplied buffer, for any buffer size | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_fmt/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
