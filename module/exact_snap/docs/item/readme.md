# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_snap`'s own
source tree — 16 instances across 6 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against every crate in
the workspace, every file and crate that uses it. Callable Kinds (Function,
Associated Function/Method) additionally carry their Caller/Callee Tree
closures.

`exact_snap` is Tier 2, depending on `exact_kind` and `exact_round`. Net-new:
no real precedent exists in the 5 original crates for `Tick`, `Lot`, or grid
snapping itself (module doc comment, `src/lib.rs:9-10`).

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function/Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the actual workspace, not assumed (OT012)?
  - [ ] Do Function/Associated Function/Method instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Are same-crate same-name methods (`Tick::new`/`Lot::new`) disambiguated by filename, never conflated?
  - [ ] Is a private helper's own call-graph hop preserved in its callers'/callees' trees rather than skipped?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 2 |
| Enum | `enum/` | 1 |
| Struct | `struct/` | 2 |
| Implementation | `implementation/` | 4 |
| Associated Function/Method | `associated_function/` | 5 |
| Function | `function/` | 2 |
| **Total** | | **16** |

9 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration, Type
Alias, Union, Constant, Static, Trait, External Block, Macro
Definition/Invocation. One private free function, `round_error_to_snap_error`
(`src/lib.rs:54`), is deliberately excluded from this count — a
private/`pub(crate)`/`pub(super)` function gets no Item Instance of its own
(`item_des.rulebook.md` line 243), but it IS a real call-graph hop and
appears as a plain `file:line` citation in both [price_snap_tick](function/001_price_snap_tick.md)'s
and [qty_snap_lot](function/002_qty_snap_lot.md)'s Callee Trees.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{ Price, Quantity } | Use Declaration | 🔄 |
| use_declaration/002 | use exact_round::Rounding | Use Declaration | 🔄 |
| enum/001 | SnapError | Enum | 🔄 |
| struct/001 | Tick | Struct | 🔄 |
| struct/002 | Lot | Struct | 🔄 |
| implementation/001 | Display for SnapError | Implementation | 🔄 |
| implementation/002 | Error for SnapError | Implementation | 🔄 |
| implementation/003 | Tick inherent impl | Implementation | 🔄 |
| implementation/004 | Lot inherent impl | Implementation | 🔄 |
| associated_function/001 | Display::fmt for SnapError | Associated Function/Method | 🔄 |
| associated_function/002 | Tick::new | Associated Function/Method | 🔄 |
| associated_function/003 | Tick::price | Associated Function/Method | 🔄 |
| associated_function/004 | Lot::new | Associated Function/Method | 🔄 |
| associated_function/005 | Lot::qty | Associated Function/Method | 🔄 |
| function/001 | price_snap_tick | Function | 🔄 |
| function/002 | qty_snap_lot | Function | 🔄 |

### Notable Findings

- **`Tick::new`/`Lot::new` collide on the same bare name** — two different
  types, same method name, in the same crate's catalog. Disambiguated with
  the established `exact_kind` convention: a `_tick`/`_lot` filename suffix
  (`associated_function/002_new_tick.md` / `004_new_lot.md`), applied
  consistently to every method on both types (including `price`/`qty`, which
  don't themselves collide) for naming uniformity within the catalog.
- **`Tick::price` and `Lot::qty` are dead code by any caller's measure** —
  the sharpest finding in this crate. Both accessors are public, re-exported
  by the facade, and completely uncalled anywhere, including by this crate's
  own `price_snap_tick`/`qty_snap_lot`, which bypass them via direct `.0`
  tuple-field access instead (legal — same defining module). Verified by
  grepping `.price()`/`.qty()` across `src/lib.rs` and `tests/snap_test.rs`:
  zero matches for either.
- **Both snap functions have zero callers anywhere in the workspace** —
  `exact_arith` re-exports both but its own facade test suite never calls
  either, the same bypass pattern already found independently in
  `exact_add`/`exact_parse`'s catalogs. All real exercise is confined to this
  crate's own `tests/snap_test.rs`.
- **`SnapError::Display` is never rendered anywhere** — mirrors the identical
  finding already recorded for `exact_kind::KindError`, `exact_ratio::RatioError`.
- **The `RoundError::DivZero` arm `round_error_to_snap_error` maps is
  currently unreachable** through this crate's own public API (`Tick::new`/
  `Lot::new` already refuse a zero-sized grid before any division happens) —
  the function's own comment (`src/lib.rs:58-62`) discloses this, calling it
  the same defensive-but-unreachable pattern `exact_kind::Decimal::checked_neg`
  already uses.
- **One identifier collision correctly excluded per OT012**: `demiurg_log`
  declares its own, wholly unrelated `Tick` struct (`Tick::begin()`), and
  `demiurg_query` has `WorldTick` — neither crate depends on `exact_snap`
  (confirmed via `Cargo.toml`), so neither appears in any File/Crate Usage
  table here.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_snap/docs/item -name '*.md' -not -name readme.md | wc -l
# → 16
```
