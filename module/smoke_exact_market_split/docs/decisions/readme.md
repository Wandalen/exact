# Decisions Doc Definition

### Scope

- **Purpose**: Record this lane's own structural design choices — ones a reader could reasonably have made the other way — so the reasoning stays attached to the crate rather than living only in a commit message.
- **Responsibility**: One flat list of this crate's own Architecture Decision Records.
- **In Scope**: Why the control arm must disagree, and why the lane is a library rather than a bare `main.rs`.
- **Out of Scope**: Decisions owned by `exact_arith` or any of the 14 leaf crates this lane merely consumes (→ each crate's own `docs/decisions/`, where present).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Control Arm Must Disagree](001_control_arm_must_disagree.md) | Why the lane asserts the exact and `f64` arms disagree, rather than merely running the exact path | 🔄 |
| 002 | [Library Not Bare Main](002_library_not_bare_main.md) | Why the lane's logic lives in `src/lib.rs`, driven by `tests/lane_test.rs`, rather than in a bare `src/main.rs` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/smoke_exact_market_split/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
