# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_minor`'s
own source tree — 13 instances across 6 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against the 3 crates
that depend on `exact_minor` (`exact_sign`, `exact_kind`, `exact_arith` — per
their own `Cargo.toml`), every file and crate that uses it. Callable Kinds
(Function, Associated Function/Method) additionally carry their Caller/
Callee Tree closures.

`exact_minor` is Tier 0 of the family — the root, depending on nothing. It
owns the backing integer width (`Backing = i64`) and offers checked and
saturating arithmetic over it, with no scale or kind attached.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item (fn, struct, enum, trait, impl, etc.) declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind. No existing standard type covers per-declaration cataloging with exhaustive call-graph and cross-crate usage evidence.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale and invariants; `docs/item/` documents where a raw Rust Item is declared and every place it is used, regardless of whether it embodies a Domain Type.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the actual workspace, not assumed (OT012)?
  - [ ] Do Function/Associated Function/Method instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Is every top-level `pub fn` classified as Function, never conflated with Associated Function/Method (no inherent `impl` block exists in this crate)?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Type Alias | `type_alias/` | 1 |
| Enum | `enum/` | 1 |
| Implementation | `implementation/` | 2 |
| Function | `function/` | 7 |
| Associated Function/Method | `associated_function/` | 1 |
| **Total** | | **13** |

Nine of the 15 taxonomy Kinds are absent from this crate: Module, Extern
Crate Declaration, Struct, Union, Static, Trait, External Block, Macro
Definition/Invocation, Associated Constant/Type. Each was checked for
systematically against `src/lib.rs` (the crate's only source file) — none
declares any instance of these. Notably absent: Struct — unlike every other
Tier-0/1 crate in the family, `exact_minor` has no struct of its own; it
operates entirely on the bare `Backing` alias and free functions.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use core::fmt | Use Declaration | 🔄 |
| type_alias/001 | Backing | Type Alias | 🔄 |
| enum/001 | MinorError | Enum | 🔄 |
| implementation/001 | Display for MinorError | Implementation | 🔄 |
| implementation/002 | Error for MinorError | Implementation | 🔄 |
| function/001 | minor_zero | Function | 🔄 |
| function/002 | minor_is_zero | Function | 🔄 |
| function/003 | minor_checked_add | Function | 🔄 |
| function/004 | minor_checked_sub | Function | 🔄 |
| function/005 | minor_checked_neg | Function | 🔄 |
| function/006 | minor_saturating_add | Function | 🔄 |
| function/007 | minor_saturating_sub | Function | 🔄 |
| associated_function/001 | Display::fmt for MinorError | Associated Function/Method | 🔄 |

### Notable Findings

- **The entire functional API has zero external callers.** Of the 7 free
  functions plus the `Display` method, none is called by `exact_sign` or
  `exact_kind` — the crate's only two real (non-facade) dependents. Both
  consumers import only the `Backing` type alias and reimplement checked
  arithmetic inline on `i64` directly, rather than delegating here.
  `exact_arith`'s facade re-exports every item but calls none of them either
  (pure `pub use`). This is the most pervasive honest-empty-tree finding in
  the family so far — grep-verified per function, not assumed.
- **Every item is now exercised by this crate's own suite.** Until
  2026-10-02, `minor_zero`, `minor_is_zero`, and `MinorError`'s `Display`/
  `Error` impls had no caller anywhere, this crate's own tests included;
  `tests/zero_test.rs` and `tests/checked_arithmetic_test.rs`'s
  `overflow_error_names_the_failed_operation` now cover them. Production
  callers are still absent — the first finding above still holds.
- **The saturating functions' absence of external callers has an
  architectural reason, not an oversight**: `exact_add`'s saturating
  arithmetic clamps to the *declared ceiling* (`Money::MAX`/`MIN`), a
  different clamp target than these functions' raw `Backing::MAX`/`MIN` — so
  `exact_add` correctly does not call them.
- **No struct in this crate** — unique among the family's Tier 0/1 crates
  read so far; `exact_minor` operates entirely on the bare `Backing` alias.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_minor/docs/item -name '*.md' -not -name readme.md | wc -l
# → 13
```
