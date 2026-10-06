# Type Doc Definition

### Scope

- **Purpose**: Define what `Money`, `Price`, and `Quantity` denote, so a caller reasons about them as one fixed-point family rather than three unrelated wrappers.
- **Responsibility**: `Decimal<const SCALE: u32>`, `Qty<const SCALE: u32>`, and the aliases built from them.
- **In Scope**: Representation, construction, and rendering.
- **Out of Scope**: The checked-operation contract (→ `invariant/`); non-negativity's own enforcement (→ `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Conserved Value Type Family](001_conserved_value_type_family.md) | `Decimal`/`Qty`/`Price` and the `Money`/`Quantity` aliases built from them | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_kind/docs/type
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
