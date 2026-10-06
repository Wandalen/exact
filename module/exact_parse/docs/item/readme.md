# Item Entity

Catalog of every Rust Item declared in `exact_parse`'s own source tree — 5
instances across 3 Item Kinds, all in `src/lib.rs` (this crate's only source
file). One file per declaration, classified by the closed Item Kind taxonomy
(`item_des.rulebook.md` OT001/OT002). Each instance records where the Item is
declared and, grep-verified against the one crate that depends on
`exact_parse` directly (`exact_arith` — per its own `Cargo.toml`), every file
and crate that uses it. The 3 Functions additionally carry their
Caller/Callee Tree closures.

`exact_parse` is Tier 2, depending on `exact_kind` (the parser each function
dispatches to) and `exact_scale` (the cross-crate consistency guard). The
module doc comment (`src/lib.rs:6-27`) discloses two deliberate omissions
from the preferred design's own type listing: no local `ParseError` (every
grammar failure already maps onto an existing `exact_kind::KindError`
variant) and no standalone `parse_reject_extra_digits` function (the guard
already lives inside `exact_kind`'s own parser).

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
  - [ ] Is a function-pointer array element (not a direct call) distinguished from a genuine call site in File Usage citations?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Constant | `constant/` | 1 |
| Function | `function/` | 3 |
| **Total** | | **5** |

12 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Struct, Enum, Union, Static, Trait, Implementation, External
Block, Macro Definition/Invocation. `exact_parse` declares no error enum of
its own by deliberate design choice (see module doc comment), which is why
Enum is absent despite every sibling Tier-2 crate having one.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{..} | Use Declaration | 🔄 |
| constant/001 | Money/exact_scale consistency assertion | Constant | 🔄 |
| function/001 | money_from_str | Function | 🔄 |
| function/002 | qty_from_str | Function | 🔄 |
| function/003 | price_from_str | Function | 🔄 |

### Notable Findings

- **None of the 3 functions have any production caller anywhere in the
  workspace** — an honest empty finding, grep-verified. `exact_arith` only
  re-exports all three names (`exact_arith/src/lib.rs:116`); the facade's own test suite
  (`exact_arith/tests/facade_test.rs`) parses values via
  `exact_kind::Decimal::parse` (`Money::parse`) directly instead of through
  this crate's wrappers — the identical bypass pattern already found in
  [exact_add](../../../exact_add/docs/item/readme.md)'s own Notable Findings.
- **The compile-time assertion is this crate's only real cross-crate
  coupling check**: it is the one place in the workspace where
  `exact_kind::Money`'s scale and `exact_scale::MONEY_SCALE` are proven
  consistent against each other, rather than merely sharing a numeric
  literal at each definition site by convention.
- **`qty_from_str` is the only function with a dedicated negative-value
  test** (`tests/from_str_test.rs:19-22`) — `money_from_str`/`price_from_str`
  have no equivalent since `Money`/`Price` admit negative values by design.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_parse/docs/item -name '*.md' -not -name readme.md | wc -l
# → 5
```
