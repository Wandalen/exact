# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Compile-time safety of the scale byte's width.
- **In Scope**: The one `exact_scale::MONEY_SCALE as u8` cast, `SCALE_BYTE`, that every `*_to_wire` function writes and every `*_from_wire` function checks against.
- **Out of Scope**: Decode-time validation of a `scale` byte against the expected scale (→ `../format/`, which documents the full round-trip and refusal guarantee).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Scale Byte Never Truncates](001_scale_byte_never_truncates.md) | `MONEY_SCALE` is guaranteed, at compile time, to fit the record's one-byte `scale` field | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_bytes/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
