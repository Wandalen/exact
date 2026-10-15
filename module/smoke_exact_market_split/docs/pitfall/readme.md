# Pitfall Doc Definition

### Scope

- **Purpose**: Document fixed defects found in this lane, per this project's bug-fix documentation convention, so the same shape of mistake is caught faster next time it appears.
- **Responsibility**: One flat list of this crate's own pitfall instances.
- **In Scope**: Defects found and fixed in this lane's own source (`src/lib.rs`).
- **Out of Scope**: Defects in `exact_arith` or any of the 14 leaf crates this lane consumes (→ each crate's own `docs/pitfall/`, where present).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Unchecked Subtraction In Demo Ledger](001_unchecked_subtraction_in_demo_ledger.md) | A bare `-` in `ledger()` that could overflow on an unrealistic `leak_minor`, fixed and regression-tested | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/smoke_exact_market_split/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
