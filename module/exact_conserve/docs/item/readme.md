# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_conserve`'s
own source tree — 20 instances across 6 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and every file and crate that uses it,
verified via a **full-workspace grep**, not just `exact_conserve`'s own
`Cargo.toml` dependents — see Notable Findings below for why that distinction
mattered here specifically. Callable Kinds (Function, Associated
Function/Method) additionally carry their Caller/Callee Tree closures.

`exact_conserve` is Tier 3, depending on `exact_kind` and `exact_add`. It
carries `exact_audit`'s whole-log auditor (`Entry`, `Report`, `verify`)
forward, now netting each asset separately, and layers a typed per-kind convenience API
(`money_conserve_into`/`qty_conserve_into`/`money_sum_assert_zero`) on top — two halves with sharply different real-world
reach, documented below.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the **full workspace**, not only direct `Cargo.toml` dependents (OT012)?
  - [ ] Do Function/Associated Function instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Where a function has a sibling with near-identical logic, is code-sharing (or its absence) checked and stated, not assumed?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 3 |
| Struct | `struct/` | 2 |
| Enum | `enum/` | 1 |
| Implementation | `implementation/` | 5 |
| Associated Function/Method | `associated_function/` | 5 |
| Function | `function/` | 4 |
| **Total** | | **20** |

9 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Union, Constant, Static, Trait, External Block, Macro
Definition/Invocation. No Associated Constant either.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{ KindError, Money, Quantity } | Use Declaration | 🔄 |
| use_declaration/002 | use std::collections::BTreeMap | Use Declaration | 🔄 |
| use_declaration/003 | use std::borrow::Borrow | Use Declaration | 🔄 |
| struct/001 | Entry | Struct | 🔄 |
| struct/002 | Report | Struct | 🔄 |
| enum/001 | ConservationError | Enum | 🔄 |
| implementation/001 | Entry inherent impl | Implementation | 🔄 |
| implementation/002 | Display for ConservationError | Implementation | 🔄 |
| implementation/003 | Error for ConservationError | Implementation | 🔄 |
| implementation/004 | Report inherent impl | Implementation | 🔄 |
| implementation/005 | Display for Report | Implementation | 🔄 |
| associated_function/001 | new for Entry | Associated Function/Method | 🔄 |
| associated_function/002 | Display::fmt for ConservationError | Associated Function/Method | 🔄 |
| associated_function/003 | is_balanced | Associated Function/Method | 🔄 |
| associated_function/004 | discrepancy_minor | Associated Function/Method | 🔄 |
| associated_function/005 | Display::fmt for Report | Associated Function/Method | 🔄 |
| function/001 | verify | Function | 🔄 |
| function/002 | money_conserve_into | Function | 🔄 |
| function/003 | qty_conserve_into | Function | 🔄 |
| function/004 | money_sum_assert_zero | Function | 🔄 |

### Notable Findings

- **The carried-forward `exact_audit` surface (`Entry`, `verify`,
  `is_balanced`) has real production callers — the first in this entire
  15-crate effort confirmed outside `module/` itself.**
  `cluster_economy/src/market.rs:507-508,518-519` calls `verify` and
  `is_balanced` directly in its settlement path, and
  `exchange_core`/`smoke_exchange_core` touch the same surface in
  integration tests and a demo lane. By contrast, the brand-new typed
  convenience layer this crate adds (`money_conserve_into`,
  `qty_conserve_into`, `money_sum_assert_zero`) has
  **zero** callers anywhere outside this crate's own tests — the same
  "never called outside the defining crate" pattern seen in every other new
  function across this migration, sitting right next to the one surface
  that breaks the pattern.
- **This catalog's own first draft under-reported `is_balanced`'s usage**,
  initially written as an honest-empty finding before a full-workspace grep
  (rather than a grep scoped to `exact_conserve`'s direct `Cargo.toml`
  dependents) surfaced the real `cluster_economy` call site. The lesson
  generalizes: a function reached only through the `exact_arith` facade can
  have real callers several hops downstream that a dependents-only sweep
  misses — every File/Crate Usage claim in this catalog was re-verified
  against a full-workspace grep before finalizing.
- **`ConservationError::Display` is actually rendered** (`exact_arith/tests/facade_test.rs:61`,
  and `exchange_core/tests/submission_test.rs:258`'s assertion-failure
  message) — the one error type in this family confirmed to have its
  `Display` output exercised, where every sibling error type elsewhere
  (`KindError`, `RatioError`, `SnapError`, `DustError`) is never rendered
  anywhere.
- **`verify` and `money_sum_assert_zero` run the same
  `i128`-accumulation fold** rather than one calling the other — `verify`
  keeps one accumulator per asset, `money_sum_assert_zero` one for the
  slice. Noted as a real, verified structural fact, not flagged as a defect
  to fix.
- **No `qty_sum_assert_zero`.** The preferred design names one, but a slice
  of non-negative quantities sums to zero only when every leg is zero, so it
  could never check a transfer; it was removed in favour of [verify](function/001_verify.md)
  with an asset key (→ [ADR-002](../decisions/002_conservation_checked_per_asset.md)).

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_conserve/docs/item -name '*.md' -not -name readme.md | wc -l
# → 20
```
