# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_round`'s
own source tree — 9 instances across 4 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against the 4 crates
that depend on `exact_round` directly (`exact_dust`, `exact_snap`,
`exact_ratio`, `exact_arith` — per their own `Cargo.toml`), every file and
crate that uses it. Callable Kinds (Function, Associated Function/Method)
additionally carry their Caller/Callee Tree closures.

`exact_round` is one of 3 Tier-0 roots with no edges to any sibling crate —
and, unlike `exact_minor`/`exact_scale` (which port real behaviour forward
from `exact_decimal`), every item here is net-new: the family's prior 5-crate
shape never offered more than one implicit rounding behaviour, so there is no
real-code precedent to catalog against, only the preferred design's own
fresh specification (module doc comment, `src/lib.rs:9-12`).

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item (fn, struct, enum, trait, impl, etc.) declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale and invariants; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the actual workspace (OT012)?
  - [ ] Do Function/Associated Function/Method instances carry both Caller Tree and Callee Tree (OT006)?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Enum | `enum/` | 2 |
| Function | `function/` | 4 |
| Implementation | `implementation/` | 2 |
| Associated Function/Method | `associated_function/` | 1 |
| **Total** | | **9** |

11 of the 15 taxonomy Kinds are absent: Use Declaration, Module, Extern Crate
Declaration, Type Alias, Struct, Union, Constant, Static, Trait, External
Block, Macro Definition/Invocation, Associated Constant/Type. `exact_round`
has no dependencies (Tier 0), so it needs no `use` declarations for anything
beyond its own items, and declares no data-carrying struct or constant of its
own — everything it exposes is policy logic (2 enums, 4 functions, their
error-rendering machinery).

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| enum/001 | Rounding | Enum | 🔄 |
| enum/002 | RoundError | Enum | 🔄 |
| function/001 | rounding_default | Function | 🔄 |
| function/002 | rounding_name | Function | 🔄 |
| function/003 | round_div | Function | 🔄 |
| function/004 | round_div_wide | Function | 🔄 |
| implementation/001 | Display for RoundError | Implementation | 🔄 |
| implementation/002 | Error for RoundError | Implementation | 🔄 |
| associated_function/001 | Display::fmt for RoundError | Associated Function/Method | 🔄 |

### Notable Findings

- **`round_div` and `round_div_wide` are the crate's load-bearing exports**:
  `round_div` has real production callers in all 3 of `exact_dust`,
  `exact_snap`, and `exact_ratio`, and `round_div_wide` in `exact_ratio`'s
  ratio multiplies — the shared rounding division the module doc comment
  explains was deliberately centralized here to avoid duplicating
  sign-handling/tie-breaking logic three times. The rounding rules
  themselves live once, in `round_div_wide`; `round_div` widens into it.
- **`rounding_default` and `rounding_name` are each unused outside their own
  tests** — both are re-exported through `exact_arith`'s facade, but no
  production or test file anywhere in the 15-crate family actually calls
  either one. Every call site that needs `HalfEven` currently writes the
  variant literally rather than calling `rounding_default()`. A real,
  individually grep-verified gap, not an omission.
- **`RoundError`'s `Display` is rendered only by this crate's own tests**,
  which pin all three messages — every consumer (`exact_dust`, `exact_snap`,
  `exact_ratio`) maps it into its own local error type via `match`, never by
  formatting the message. Same pattern as
  [`exact_kind::KindError`](../../../exact_kind/docs/item/enum/001_kind_error.md).

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_round/docs/item -name '*.md' -not -name readme.md | wc -l
# → 9
```
