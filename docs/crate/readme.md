# Crate Doc Definition

### Scope

- **Purpose**: Record the proposed crate-level dependency graph for workstream 006's preferred 15-crate decomposition, so the target shape traces back to its source conversation.
- **Responsibility**: Each of the 15 proposed crates' `Deps`/`Boundary` fields.
- **In Scope**: `exact_minor`, `exact_scale`, `exact_kind`, `exact_sign`, `exact_round`, `exact_add`, `exact_ratio`, `exact_dust`, `exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_conserve`, `exact_cmp`, `exact_arith` dependency edges.
- **Out of Scope**: The real, implemented crates' actual dependencies (→ each crate's own `Cargo.toml`); this family's proposed struct/enum/function surface (→ `../type/`); workstream 002's own dependency rules and any other workstream's scope boundaries — those belong to their own workstreams' docs, not here, even where they name this family in passing.

**Design status**: implemented. All 15 crates below now exist in `module/`, alongside the demo lane — the real implementation is `exact_minor`, `exact_scale`, `exact_kind`, `exact_sign`, `exact_round`, `exact_add`, `exact_ratio`, `exact_dust`, `exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_conserve`, `exact_cmp`, `exact_arith`, and `smoke_exact_market_split`. The old 5-crate implementation (`exact_decimal`, `exact_qty`, `exact_audit`, `exact_arithmetic`, `smoke_exact_arithmetic`) was deleted at cutover and no longer exists on disk (retrievable via `git show HEAD:<old_crate>/<path>`). See each crate's own instance above for whether its Deps/Boundary proposal was followed as specified or where it deviated.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [exact_minor](001_exact_minor.md) | The raw integer subunit; root of the dependency tree | 🔄 |
| 002 | [exact_scale](002_exact_scale.md) | Digits-after-point and same-scale check; root of the dependency tree | 🔄 |
| 003 | [exact_kind](003_exact_kind.md) | Money/Qty/Price distinct newtypes; depends on minor, scale | 🔄 |
| 004 | [exact_sign](004_exact_sign.md) | Zero/positive/negative, no negative zero; depends on minor | 🔄 |
| 005 | [exact_round](005_exact_round.md) | Rounding mode as data; root of the dependency tree | 🔄 |
| 006 | [exact_add](006_exact_add.md) | Checked/saturating add and sub; depends on kind, sign | 🔄 |
| 007 | [exact_ratio](007_exact_ratio.md) | Multiply by n/d, divide with a mode; depends on kind, round | 🔄 |
| 008 | [exact_dust](008_exact_dust.md) | Last-subunit destination; depends on kind, ratio | 🔄 |
| 009 | [exact_parse](009_exact_parse.md) | "1.23" to minor units or error; depends on kind, scale | 🔄 |
| 010 | [exact_fmt](010_exact_fmt.md) | Print with scale, storage stays integer; depends on kind | 🔄 |
| 011 | [exact_bytes](011_exact_bytes.md) | Wire and save as minor plus scale; depends on kind, scale | 🔄 |
| 012 | [exact_snap](012_exact_snap.md) | Snap price to tick, quantity to lot; depends on kind, round | 🔄 |
| 013 | [exact_conserve](013_exact_conserve.md) | A slice of legs sums to zero; depends on add, kind | 🔄 |
| 014 | [exact_cmp](014_exact_cmp.md) | Exact order, no epsilon; depends on kind | 🔄 |
| 015 | [exact_arith](015_exact_arith.md) | Facade re-exporting all 14 leaves; depends on all of them | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/crate
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              15
# rows in Overview Table: 15
```
