# Item Entity

Catalog of every Rust Item declared in `exact_sign`'s own source tree — 6
instances across 3 Item Kinds, all in `src/lib.rs` (this crate's only source
file). One file per declaration, classified by the closed Item Kind taxonomy
(`item_des.rulebook.md` OT001/OT002). Each instance records where the Item is
declared and, grep-verified against the 2 crates that depend on `exact_sign`
directly (`exact_add`, `exact_arith` — per their own `Cargo.toml`), every
file and crate that uses it. The 4 Functions additionally carry their
Caller/Callee Tree closures.

`exact_sign` is Tier 1, depending on `exact_minor` alone. Net-new: the
family's prior shape encoded "can this go negative" as a type-level choice
rather than a runtime policy value, so there is no real-code precedent to
port (module doc comment, `src/lib.rs:6-10`).

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function kind additionally requires Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified (OT012)?
  - [ ] Do Function instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Does a module doc comment's claimed usage get cross-checked against actual call sites, not taken on faith?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Enum | `enum/` | 1 |
| Function | `function/` | 4 |
| **Total** | | **6** |

12 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Struct, Union, Constant, Static, Trait, Implementation, External
Block, Macro Definition/Invocation. `exact_sign` declares no hand-written
`impl` block (`Sign` only derives), which is why Implementation is absent
despite the crate having an enum.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_minor::Backing | Use Declaration | 🔄 |
| enum/001 | Sign | Enum | 🔄 |
| function/001 | sign_of | Function | 🔄 |
| function/002 | sign_is_negative | Function | 🔄 |
| function/003 | sign_is_zero | Function | 🔄 |
| function/004 | sign_neg_allowed | Function | 🔄 |

### Notable Findings

- **`sign_neg_allowed` is documented as wired into `exact_kind` but is not.**
  Its own doc comment claims `exact_kind` calls it once per kind at
  construction; `exact_kind` has no dependency on `exact_sign` at all, and
  enforces non-negativity directly instead. `exact_kind`'s own decisions doc
  already discloses this gap accurately — see
  [sign_neg_allowed](function/004_sign_neg_allowed.md) for the full
  cross-check. The function has zero callers anywhere in the workspace.
- **`sign_is_zero` also has zero external callers** — `exact_minor` solves the
  same question independently (`minor_is_zero`) rather than depending on this
  sibling Tier-1 crate.
- **`sign_is_negative` is the crate's one load-bearing export**: `exact_add`'s
  `money_saturating_add` calls it in production to pick a clamp direction.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_sign/docs/item -name '*.md' -not -name readme.md | wc -l
# → 6
```
