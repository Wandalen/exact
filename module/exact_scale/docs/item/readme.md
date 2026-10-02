# Item Entity

Catalog of every Rust Item declared in `exact_scale`'s own source tree — 6
instances across 2 Item Kinds, all in `src/lib.rs` (this crate's only source
file). `exact_scale` is a Tier 0 root (no dependencies of its own) holding
the power-of-ten table and the declared ceiling's constants. One file per
declaration, classified by the closed Item Kind taxonomy (`item_des.rulebook.md`
OT001/OT002). Each instance records where the Item is declared and, grep-
verified against the 4 crates that depend on `exact_scale` directly
(`exact_parse`, `exact_bytes`, `exact_kind`, `exact_arith` — per their own
`Cargo.toml`), every file and crate that uses it. The one Function
(`pow10`) additionally carries its Caller/Callee Tree closure.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item (fn, struct, enum, trait, impl, etc.) declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind. No existing standard type covers per-declaration cataloging with exhaustive call-graph and cross-crate usage evidence.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale and invariants; `docs/item/` documents where a raw Rust Item is declared and every place it is used, regardless of whether it embodies a Domain Type.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the actual workspace, not assumed (OT012)?
  - [ ] Does the Function instance carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Is the anonymous (`_`-named) compile-time assertion constant cataloged alongside the named constants, since Eligibility does not require a usable identifier (OT004)?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Constant | `constant/` | 5 |
| Function | `function/` | 1 |
| **Total** | | **6** |

Thirteen of the 15 taxonomy Kinds are absent from this crate: Use
Declaration, Module, Extern Crate Declaration, Type Alias, Struct, Enum,
Union, Static, Implementation, Trait, External Block, Macro
Definition/Invocation, and both Associated Item Kinds. `exact_scale` is a
Tier 0 root with zero dependencies — it has no `use` declaration of its own,
consistent with its own module doc comment's point that Tier 0 crates "have
no edges to one another by design."

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| constant/001 | HEADROOM_FACTOR | Constant | 🔄 |
| constant/002 | CEILING_WHOLE_UNITS | Constant | 🔄 |
| constant/003 | MONEY_SCALE | Constant | 🔄 |
| constant/004 | CEILING_MINOR_UNITS | Constant | 🔄 |
| constant/005 | (anonymous range-budget assertion) | Constant | 🔄 |
| function/001 | pow10 | Function | 🔄 |

### Notable Findings

- **`MONEY_SCALE` is the crate's single most consumed item** — it reaches
  production code in 3 of the 4 dependent crates (`exact_bytes`'s wire
  format, `exact_kind`'s type aliases, `exact_parse`'s consistency assert),
  more than any other constant here.
- **`HEADROOM_FACTOR` and `CEILING_WHOLE_UNITS` have no production consumer
  outside this crate** — both are checked once at declaration (the range-
  budget assert) and otherwise only re-verified by tests. An honest gap, not
  an omission: the headroom relation is a compile-time invariant, not a
  runtime dependency anything needs to read.
- **`pow10`'s Caller Tree is mostly const-evaluation contexts**, not ordinary
  function-body calls — 3 of its 4 call sites are other constants' own
  initializer expressions (`CEILING_MINOR_UNITS` here, `exact_parse`'s
  consistency assert, `exact_kind::Decimal::ONE_MINOR`). Only
  `exact_kind::Decimal::parse`'s fractional-digit scaling calls it at
  ordinary runtime.
- **The anonymous assertion constant (`constant/005`) has no outside
  references by construction** — it is both private and unnamed (`_`), so no
  file anywhere can refer to it; `exact_kind`'s test suite independently
  re-states the same check rather than importing it, since there is nothing
  importable.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_scale/docs/item -name '*.md' -not -name readme.md | wc -l
# → 6
```
