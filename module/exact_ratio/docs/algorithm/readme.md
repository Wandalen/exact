# Algorithm Doc Definition

### Scope

- **Purpose**: State exactly how this crate's operations compute their result, so a caller can predict the output — and the failure mode — of a call without reading the implementation.
- **Responsibility**: The widened multiply every `*_mul_ratio` function shares.
- **In Scope**: The procedure's steps and its correctness argument.
- **Out of Scope**: The division `div_round` drives, owned by [`exact_round`](../../../exact_round/readme.md); the shape of `Ratio` itself (→ [`type/`](../type/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Widened Multiply Before Narrow](001_widened_multiply_before_narrow.md) | Why the product is computed in `i128` before narrowing | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_ratio/docs/algorithm
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
