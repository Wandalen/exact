# Algorithm Doc Definition

### Scope

- **Purpose**: Index every procedure this crate implements, so a reader can find the algorithm behind any function without reading the source cold.
- **Responsibility**: One instance per distinct procedure this crate's public functions carry out.
- **In Scope**: Per-kind text-parsing dispatch and the grammar it inherits from `exact_kind`.
- **Out of Scope**: The grammar's own implementation, owned by `exact_kind`; rendering (→ [`exact_fmt`'s `algorithm/`](../../../exact_fmt/docs/algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Decimal Parsing Per Kind](001_decimal_parsing_per_kind.md) | Text-to-value dispatch for `Money`/`Quantity`/`Price`, and the grammar it inherits | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_parse/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
