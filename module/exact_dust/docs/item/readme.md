# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_dust`'s own
source tree — 13 instances across 5 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against every crate
that depends on `exact_dust` — directly (`exact_arith`) or transitively
through it (`smoke_exact_market_split`, confirmed via its own `Cargo.toml`
rather than assumed) — every file and crate that uses it. Function and
Associated Function/Method kinds additionally carry their Caller/Callee Tree
closures.

`exact_dust` is Tier 3, depending on `exact_kind` and `exact_round` directly
— the module doc comment (`src/lib.rs:12-19`) discloses this diverges from
the preferred design's own `exact_kind, exact_ratio` listing, matching
`exact_snap`'s identical precedent for reaching `round_div` without an
unused dependency on `exact_ratio`'s rational-multiplier surface.

**Four private top-level functions are real call-graph hops, cataloged
nowhere of their own** (`item_des.rulebook.md` § Instance Documentation :
Caller Tree Content): `round_error_to_dust_error` (`src/lib.rs:103`),
`split_minor` (`118`), `slot_minor` (`135`), and `fill_minor` (`149`).
`split_minor` is called by all 6 public functions; `slot_minor` by the 2
`*_split_into` functions directly and by `fill_minor`; `fill_minor` only by
the 2 `*_split` functions — never by the 2 `*_remainder` functions — each function's own
Callee Tree reflects this exactly rather than repeating one copy-pasted
shape across all 6.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified, including transitive (facade-reached) dependents (OT012)?
  - [ ] Do Function/Associated Function instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Does each function's Callee Tree reflect which private helpers it *actually* calls, rather than a uniform copy-paste across similarly-shaped functions?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 2 |
| Enum | `enum/` | 2 |
| Implementation | `implementation/` | 2 |
| Associated Function/Method | `associated_function/` | 1 |
| Function | `function/` | 6 |
| **Total** | | **13** |

10 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Struct, Union, Constant, Static, Trait, External Block, Macro
Definition/Invocation. No Associated Constant either, and no struct — `DustTo`
and `DustError` are both plain enums with no inherent `impl` of their own,
only the two trait impls on `DustError`.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{ Money, Quantity } | Use Declaration | 🔄 |
| use_declaration/002 | use exact_round::{ RoundError, Rounding } | Use Declaration | 🔄 |
| enum/001 | DustTo | Enum | 🔄 |
| enum/002 | DustError | Enum | 🔄 |
| implementation/001 | Display for DustError | Implementation | 🔄 |
| implementation/002 | Error for DustError | Implementation | 🔄 |
| associated_function/001 | Display::fmt for DustError | Associated Function/Method | 🔄 |
| function/001 | money_dust_split | Function | 🔄 |
| function/002 | money_dust_split_into | Function | 🔄 |
| function/003 | money_dust_remainder | Function | 🔄 |
| function/004 | qty_dust_split | Function | 🔄 |
| function/005 | qty_dust_split_into | Function | 🔄 |
| function/006 | qty_dust_remainder | Function | 🔄 |

### Notable Findings

- **`money_dust_split` is the one function in this entire crate with a real
  production caller** — `smoke_exact_market_split::market_split`
  (`smoke_exact_market_split/src/lib.rs:142`), reached through the
  `exact_arith` facade re-export and confirmed via that crate's own
  `Cargo.toml` dependency, not assumed from the name match alone. It's also
  the one function `exact_arith`'s own crate-doc doctest and test suite
  genuinely call — unlike every sibling catalog built so far in this family
  (`exact_add`/`exact_parse`/`exact_cmp`/`exact_snap`), where the facade's
  test bypasses the re-export entirely and calls `exact_kind` directly
  instead.
- **`qty_dust_split_into` has zero references anywhere** beyond its own
  declaration and the facade re-export — not even exercised by this crate's
  own test suite, the only one of the 6 functions with that gap. Its
  `Money`-side counterpart (`money_dust_split_into`) does have a test.
- **`DustError::Display` is never rendered anywhere**, the same pattern
  already recorded for `KindError`/`RatioError`/`SnapError`/`FmtError`
  across this family — every error type gets a real `Display` impl that
  nothing actually formats; every real consumer matches the variant or
  discards the error via `.expect(...)`.
- **The leftover is computed at the raw minor-unit level, not through each
  kind's checked arithmetic** — a deliberate choice disclosed in the module
  doc comment (`src/lib.rs:36-45`) and worth citing on its own terms:
  an `Up`/`HalfEven` rounding mode can make the per-share allocation exceed
  the total, requiring slot 0 to be *decreased*, which `Qty::checked_add`
  cannot express at all.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_dust/docs/item -name '*.md' -not -name readme.md | wc -l
# → 13
```
