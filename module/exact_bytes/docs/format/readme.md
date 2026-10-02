# Format Doc Definition

### Scope

- **Purpose**: Fix the on-the-wire byte layout this crate reads and writes, so an encoder and a decoder built independently against this document agree.
- **Responsibility**: `Wire`'s 10-byte record.
- **In Scope**: Field order, widths, byte order, and the round-trip guarantee.
- **Out of Scope**: What a decoded value becoming a specific kind can fail with (→ [`decisions/`](../decisions/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Wire Record Encoding](001_wire_record_encoding.md) | The 10-byte minor/scale/kind layout and its round-trip guarantee | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_bytes/docs/format
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
