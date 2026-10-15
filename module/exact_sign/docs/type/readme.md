# Type Doc Definition

### Scope

- **Purpose**: Define the sign-classification vocabulary this crate exports, so a caller reasons in terms of `Sign` rather than re-deriving a zero comparison.
- **Responsibility**: The `Sign` enum and its classification functions.
- **In Scope**: Classification of a single `Backing` value.
- **Out of Scope**: The negative-admission policy built on top of classification (→ `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Sign Classification](001_sign_classification.md) | `Sign` (`Neg`/`Zero`/`Pos`) and the three functions classifying a `Backing` value against it | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_sign/docs/type
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
