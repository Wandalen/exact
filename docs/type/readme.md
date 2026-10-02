# Type Doc Definition

### Scope

- **Purpose**: Document the proposed struct/enum/trait/function/error surface for each of workstream 006's preferred 15 crates, so the target API shape is traceable to its source conversation.
- **Responsibility**: One instance per proposed crate, covering its full code surface together (types, functions, and errors are inseparable in the source material).
- **In Scope**: The 15 proposed crates' types, functions, traits, and error enums as specified in the source.
- **Out of Scope**: The real, implemented crates' actual code surface (→ each crate's own `docs/type/`); the proposed crates' dependency edges (→ `../crate/`).

**Design status**: implemented. All 15 crates below now exist in `module/` — see each crate's own instance above for whether its proposed struct/function/error surface was followed as specified or where it deviated, and the real crate's own `docs/decisions/`, `docs/type/`, or `docs/algorithm/` for the full account.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [exact_minor Types](001_exact_minor_types.md) | `Minor(i64)`, `MinorWide(i128)`, checked/saturating arithmetic fns, `MinorError` | 🔄 |
| 002 | [exact_scale Types](002_exact_scale_types.md) | `Scale(u8)`, `SCALE_MAX`, scale fns, `ScaleError` | 🔄 |
| 003 | [exact_kind Types](003_exact_kind_types.md) | `Money`/`Qty`/`Price`, conversion fns, `KindError`, trait `Scaled` | 🔄 |
| 004 | [exact_sign Types](004_exact_sign_types.md) | `Sign` enum, sign fns, `SignError` | 🔄 |
| 005 | [exact_round Types](005_exact_round_types.md) | `Rounding` enum, rounding fns | 🔄 |
| 006 | [exact_add Types](006_exact_add_types.md) | add/sub fns per kind, `AddError` | 🔄 |
| 007 | [exact_ratio Types](007_exact_ratio_types.md) | `Ratio`, mul_ratio/div_round fns, `RatioError` | 🔄 |
| 008 | [exact_dust Types](008_exact_dust_types.md) | `DustTo` enum, dust_split fns, `DustError` | 🔄 |
| 009 | [exact_parse Types](009_exact_parse_types.md) | from_str fns per kind, `ParseError` | 🔄 |
| 010 | [exact_fmt Types](010_exact_fmt_types.md) | fmt fns per kind, `FmtError`, `Display` wrapper | 🔄 |
| 011 | [exact_bytes Types](011_exact_bytes_types.md) | `Wire`, to/from_wire fns, `WireError` | 🔄 |
| 012 | [exact_snap Types](012_exact_snap_types.md) | `Tick`, `Lot`, snap fns, `SnapError` | 🔄 |
| 013 | [exact_conserve Types](013_exact_conserve_types.md) | sum_assert_zero/conserve_into fns, `ConservationError` | 🔄 |
| 014 | [exact_cmp Types](014_exact_cmp_types.md) | cmp/eq/min/max fns, `CmpError` | 🔄 |
| 015 | [exact_arith Types](015_exact_arith_types.md) | Re-exports, `exact_zero_*` fns, no new errors | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/docs/type
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              15
# rows in Overview Table: 15
```
