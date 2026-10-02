# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: Totality and consistency of the ordering `money_cmp`/`qty_cmp`/`price_cmp` provide.
- **In Scope**: Reflexivity, antisymmetry, transitivity, and the absence of any incomparable case.
- **Out of Scope**: Why no `CmpError`/conditional `Ord` exists at all (→ `../decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Ordering Is Total, With No Incomparable Case](001_ordering_is_total_with_no_incomparable_case.md) | Every pair of same-kind values has a definite, consistent order — no `NaN`-like exception | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_cmp/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
