# Item Entity

Catalog of every Rust Item declared in `exact_cmp`'s own source tree — 7
instances across 2 Item Kinds, all in `src/lib.rs` (this crate's only source
file). One file per declaration, classified by the closed Item Kind taxonomy
(`item_des.rulebook.md` OT001/OT002). Each instance records where the Item is
declared and, grep-verified against the one crate that depends on `exact_cmp`
directly (`exact_arith` — per its own `Cargo.toml`), every file and crate
that uses it. The 6 Functions additionally carry their Caller/Callee Tree
closures.

`exact_cmp` is Tier 2, depending on `exact_kind` alone. Net-new free-function
wrapper over functionality `exact_kind`'s `Decimal`/`Qty` already derive
(`Ord`, `PartialEq`) — no real precedent to port; the module doc comment
(`src/lib.rs:7-18`) discloses why this crate carries no local `CmpError` and
no conditional `Ord`.

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
  - [ ] Is the crate's actual per-kind operation coverage (which kinds get `_cmp`/`_eq`/`_min`/`_max`) stated as a verified fact, not assumed symmetric?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Function | `function/` | 6 |
| **Total** | | **7** |

13 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Struct, Enum, Union, Constant, Static, Trait, Implementation,
External Block, Macro Definition/Invocation. `exact_cmp` declares no
hand-written `impl` block and no local error enum (see the crate's own
disclosed deviation), which is why Implementation and Enum are both absent.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{ Money, Price, Quantity } | Use Declaration | 🔄 |
| function/001 | money_cmp | Function | 🔄 |
| function/002 | qty_cmp | Function | 🔄 |
| function/003 | price_cmp | Function | 🔄 |
| function/004 | money_eq | Function | 🔄 |
| function/005 | price_min | Function | 🔄 |
| function/006 | price_max | Function | 🔄 |

### Notable Findings

- **Coverage is asymmetric across kinds, confirmed real rather than an
  oversight**: `_cmp` exists for all three kinds (`money_cmp`, `qty_cmp`,
  `price_cmp`), but `_eq` exists only for `Money` (`money_eq`) and
  `_min`/`_max` only for `Price` (`price_min`, `price_max`). An exhaustive
  grep of `src/lib.rs` turned up no `qty_eq`, `price_eq`, `money_min`,
  `money_max`, `qty_min`, or `qty_max` — the gap is the crate's actual shipped
  surface, not a missed catalog entry.
- **Every one of the 6 functions has an honest empty Caller Tree.** All 6 are
  re-exported by `exact_arith` (`src/lib.rs:119`) but never called by the
  facade's own test or by any other crate in `substrate/` or `module/` —
  confirmed via `grep -rn '<fn>(' --include='*.rs'` across the full
  workspace for each function individually. Every call site that exists
  anywhere is inside `exact_cmp`'s own `tests/cmp_test.rs`.
- **Every function is a one-line dispatch to `exact_kind`'s derived traits**
  (`Ord`/`PartialEq`) — none contains independent comparison logic, matching
  the module doc comment's own disclosed reasoning for why no `CmpError` or
  scale-mismatch guard exists under this family's const-generic
  representation.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_cmp/docs/item -name '*.md' -not -name readme.md | wc -l
# → 7
```
