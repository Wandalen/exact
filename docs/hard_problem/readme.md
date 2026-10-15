# Hard Problem Doc Definition

### Scope

- **Purpose**: Document the "hard problems" workstream 006's preferred design exists to solve, so the requirement each proposed feature/crate serves is traceable.
- **Responsibility**: The 14 hard problems enumerated in `codename_vm_ecs_export_1.md`'s Prompt-1 answer.
- **In Scope**: The correctness/behavior mandates the target `exact_*` family must satisfy.
- **Out of Scope**: The features that satisfy each requirement (→ `../feature/`); the crates that own each requirement (→ `../crate/`).

**Design status**: realized across the real 15-crate family (`exact_minor`, `exact_scale`, `exact_kind`, `exact_sign`, `exact_round`, `exact_add`, `exact_ratio`, `exact_dust`, `exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_conserve`, `exact_cmp`, `exact_arith`) plus the `smoke_exact_market_split` demo lane — not the old 5-crate implementation (`exact_decimal`/`exact_qty`/`exact_audit`/`exact_arithmetic`/`smoke_exact_arithmetic`) this corpus originally named. Every hard problem below traces to at least one real crate, and most hold exactly as stated; two resolved differently than proposed — [013](013_scale_mismatch.md) is avoided by a compile-time design choice rather than handled at runtime, and [012](012_hot_path_performance.md)'s own proposed verification (a benchmark vs `f64`) was never built — see each instance's own Design status line for the full account.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Float Money Is Wrong](001_float_money_is_wrong.md) | `0.1 + 0.2` must not run a market | 🔄 |
| 002 | [One Canonical Subunit](002_one_canonical_subunit.md) | Every caller shares one scale, not an implicit convention | 🔄 |
| 003 | [Distinct Kinds](003_distinct_kinds.md) | Money, quantity, and price are not interchangeable | 🔄 |
| 004 | [Overflow Handling](004_overflow_handling.md) | Add/multiply fail or follow a named policy, never wrap | 🔄 |
| 005 | [Division And Rounding](005_division_and_rounding.md) | Fees and splits use an explicit rounding mode | 🔄 |
| 006 | [Dust Destination](006_dust_destination.md) | The last subunit has a defined destination | 🔄 |
| 007 | [Determinism](007_determinism.md) | Same operations, same bits, on every platform | 🔄 |
| 008 | [Display Is Not Storage](008_display_is_not_storage.md) | Store an integer; print with a scale | 🔄 |
| 009 | [Sign And Zero](009_sign_and_zero.md) | Zero, positive, negative are defined; no negative zero | 🔄 |
| 010 | [Closed Vm Types](010_closed_vm_types.md) | A balance is plain bits, snapshottable and relocatable | 🔄 |
| 011 | [Tick And Lot Snap](011_tick_and_lot_snap.md) | Price and quantity snap to an instrument grid | 🔄 |
| 012 | [Hot Path Performance](012_hot_path_performance.md) | Add, compare, and ratio are cheap enough for matching | 🔄 |
| 013 | [Scale Mismatch](013_scale_mismatch.md) | Operating across two scales is an error unless converted | 🔄 |
| 014 | [Serialization](014_serialization.md) | Wire and save are integer plus scale, never `f64` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/hard_problem
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              14
# rows in Overview Table: 14
```
