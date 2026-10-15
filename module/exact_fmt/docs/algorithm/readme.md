# Algorithm Doc Definition

### Scope

- **Purpose**: Index every procedure this crate implements, so a reader can find the algorithm behind any function without reading the source cold.
- **Responsibility**: One instance per distinct procedure this crate's public functions carry out.
- **In Scope**: Per-kind rendering dispatch and the allocation-free buffer-writing primitive.
- **Out of Scope**: `Decimal`/`Qty`'s own `Display` implementation, owned by `exact_kind`; parsing (→ [`exact_parse`'s `algorithm/`](../../../exact_parse/docs/algorithm/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Decimal Rendering Per Kind](001_decimal_rendering_per_kind.md) | Value-to-text dispatch for `Money`/`Quantity`/`Price`, plus `fmt_into`'s allocation-free buffer write | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_fmt/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
