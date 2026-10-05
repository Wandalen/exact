# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_ratio`'s
own source tree — 17 instances across 6 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against the one crate
that depends on `exact_ratio` directly (`exact_arith` — per its own
`Cargo.toml`), every file and crate that uses it. Function and Associated
Function/Method instances additionally carry their Caller/Callee Tree
closures.

`exact_ratio` is Tier 2, depending on `exact_kind` and `exact_round`. Net-new:
no real precedent exists for either rational-multiply or mode-driven integer
division (module doc comment, `src/lib.rs:9-12`). The crate also declares 3
private free functions (`kind_error_to_ratio_error`, `mul_ratio_minor`,
`div_round_minor`) that get **no Item Instance of their own**
(`item_des.rulebook.md` line 243: a private/`pub(crate)`/`pub(super)` helper
is a real call-graph hop, not a catalogable Item) — they appear only as plain
`file:line` citations inside the Callee Trees of the public functions that
call them.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified (OT012)?
  - [ ] Do Function/Associated Function instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Are private helper functions correctly excluded from their own Item Instance while still appearing as real hops in Caller/Callee Trees?
  - [ ] Are derive-produced impls correctly excluded (only hand-written `impl` blocks are cataloged)?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 2 |
| Enum | `enum/` | 1 |
| Struct | `struct/` | 1 |
| Implementation | `implementation/` | 3 |
| Function | `function/` | 7 |
| Associated Function/Method | `associated_function/` | 3 |
| **Total** | | **17** |

Nine of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Union, Constant, Static, Trait, External Block, Macro
Definition/Invocation. Associated Constant and Associated Type (both
Associated Item Kinds) are also absent — neither `impl` block declares one.
`RatioError`'s and `Ratio`'s 11 combined derived-trait impls
(`Debug, Clone, Copy, PartialEq, Eq, Hash` across both types) are compiler-
generated, not hand-written `impl` blocks, and are not cataloged.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{..} | Use Declaration | 🔄 |
| use_declaration/002 | use exact_round::Rounding | Use Declaration | 🔄 |
| enum/001 | RatioError | Enum | 🔄 |
| struct/001 | Ratio | Struct | 🔄 |
| implementation/001 | Display for RatioError | Implementation | 🔄 |
| implementation/002 | Error for RatioError | Implementation | 🔄 |
| implementation/003 | Ratio inherent impl | Implementation | 🔄 |
| function/001 | ratio_new | Function | 🔄 |
| function/002 | money_mul_ratio | Function | 🔄 |
| function/003 | qty_mul_ratio | Function | 🔄 |
| function/004 | price_mul_ratio | Function | 🔄 |
| function/005 | money_div_round | Function | 🔄 |
| function/006 | qty_div_round | Function | 🔄 |
| function/007 | price_mul_qty | Function | 🔄 |
| associated_function/001 | Display::fmt for RatioError | Associated Function/Method | 🔄 |
| associated_function/002 | Ratio::n | Associated Function/Method | 🔄 |
| associated_function/003 | Ratio::d | Associated Function/Method | 🔄 |

### Notable Findings

- **Almost no intra-workspace callers.** The one production call between
  this crate's own functions is [price_mul_qty](function/007_price_mul_qty.md)
  calling [ratio_new](function/001_ratio_new.md); outside the crate, only
  `exact_arith` calls into it — `price_mul_qty`, in its crate-doc example and
  `tests/facade_test.rs`. No other workspace crate depends on `exact_ratio`
  at all today (`grep -rl exact_ratio --include=Cargo.toml` returns only
  `exact_ratio` and `exact_arith`). Every function is exercised by
  `exact_ratio`'s own 28-test suite.
- **The plan's Tier 3 dependency table is stale relative to what was actually
  built.** The migration plan lists `exact_dust` as depending on
  `exact_kind, exact_ratio`; the real `exact_dust/Cargo.toml` depends on
  `exact_kind, exact_round` directly, bypassing this crate entirely —
  confirmed both by `Cargo.toml` and by `exact_dust`'s own disclosed
  deviation (`exact_dust/src/lib.rs:12-19`): "the actual need is the
  integer-count, mode-driven division `exact_snap` already depends on
  `exact_round` directly for — not `exact_ratio`'s rational multiplier
  surface." Two independent sources (the dependency graph and the crate's
  own self-disclosure) agree.
- **`RatioError`'s `Display` is never rendered anywhere** in the current
  workspace — confirmed by grep, mirroring `exact_kind::KindError`'s
  identical finding. Every call site matches `RatioError` by variant.
- **`Ratio::n`/`Ratio::d` are never called in production, even internally** —
  `mul_ratio_minor`, the one place a `Ratio`'s fields are read, accesses the
  private fields directly rather than through the public accessors, since
  both are defined in the same module.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_ratio/docs/item -name '*.md' -not -name readme.md | wc -l
# → 17 (excludes the 3 private helper functions, which get no Item Instance)
```
