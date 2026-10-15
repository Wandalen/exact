# Decisions Doc Definition

### Scope

- **Purpose**: Record the judgment calls behind this crate's design, so a later reader finds the reasoning instead of re-deriving or second-guessing it.
- **Responsibility**: Why a tick and a lot have the shape they have.
- **In Scope**: Decisions closed by this crate itself.
- **Out of Scope**: The family-wide change to generic operations (→ [`exact_arith`'s decisions](../../../exact_arith/docs/decisions/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [A Tick And A Lot Carry Their Own Scale](001_tick_and_lot_generic_over_scale.md) | Why `Tick`/`Lot` became aliases of `TickAt< S >`/`LotAt< S >`, and why `exact_snap` now depends on `exact_scale` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_snap/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
