# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_fmt`'s own
source tree — 12 instances across 6 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against the 2 crates
that depend on `exact_fmt` directly (`exact_fmt` and `exact_arith`, per their
own `Cargo.toml`), every file and crate that uses it. Callable Kinds
(Function, Associated Function/Method) additionally carry their
Caller/Callee Tree closures.

`exact_fmt` is Tier 2, depending on `exact_kind` alone. Its module doc
comment (`src/lib.rs:7-25`) discloses a real constraint this catalog treats
as load-bearing content: the preferred design wanted `Display` to live here
as a thin wrapper over `fmt_into`, which Rust's orphan rules make impossible
given this migration's chosen dependency direction (`exact_fmt` depends on
`exact_kind`, so neither the trait nor the type is local to this crate).
`Display` stays on `exact_kind::Decimal`/`Qty` instead; this crate layers the
preferred design's per-kind names and the `fmt_into` buffer-writing
primitive over that existing impl.

**One private top-level Item is cataloged**: `struct ByteBufWriter` has no
`pub` modifier, but `item_des.rulebook.md`'s only explicit visibility
carve-out exempts private *functions* specifically (§ Instance Documentation
: Caller Tree Content) — nothing narrows the Item Entity's "every Rust
Item... defined in this crate's own source tree" scope by visibility for any
other Kind. See [ByteBufWriter](struct/001_byte_buf_writer.md) for the full
reasoning. **One function-body-local `use` is deliberately excluded**:
`fmt_into`'s internal `use core::fmt::Write;` (`src/lib.rs:97`) is not a
top-level Item (§ Item Kind Taxonomy : Stable Item Kinds: "every **top-level**
Rust Item") — see [fmt_into](function/001_fmt_into.md) for the citation.

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
  - [ ] Is a private top-level Item (not a function) still cataloged, and a function-body-local declaration correctly excluded as non-top-level?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Enum | `enum/` | 1 |
| Struct | `struct/` | 1 |
| Implementation | `implementation/` | 3 |
| Associated Function/Method | `associated_function/` | 2 |
| Function | `function/` | 4 |
| **Total** | | **12** |

9 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Union, Constant, Static, Trait, External Block, Macro
Definition/Invocation. No Associated Constant either — `Wire`-style
`ENCODED_LEN`-type constants don't appear in this crate.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{ Money, Price, Quantity } | Use Declaration | 🔄 |
| enum/001 | FmtError | Enum | 🔄 |
| struct/001 | ByteBufWriter | Struct | 🔄 |
| implementation/001 | Display for FmtError | Implementation | 🔄 |
| implementation/002 | Error for FmtError | Implementation | 🔄 |
| implementation/003 | Write for ByteBufWriter | Implementation | 🔄 |
| associated_function/001 | Display::fmt for FmtError | Associated Function/Method | 🔄 |
| associated_function/002 | write_str for ByteBufWriter | Associated Function/Method | 🔄 |
| function/001 | fmt_into | Function | 🔄 |
| function/002 | money_fmt | Function | 🔄 |
| function/003 | qty_fmt | Function | 🔄 |
| function/004 | price_fmt | Function | 🔄 |

### Notable Findings

- **Every function is exercised by this crate's own tests** —
  `money_fmt`, `qty_fmt`, `price_fmt` and `fmt_into` all appear in
  `tests/fmt_test.rs`, even where they have zero external callers.
- **`fmt_into` is the crate's one genuinely load-bearing export** — every
  other function is a thin `to_string()` wrapper, but `fmt_into` is the
  actual reason the crate exists (non-allocating rendering). It has zero
  external callers today, but is fully exercised by 6 of its own tests.
- **`FmtError`'s `Display` is never rendered anywhere**, mirroring the
  identical finding already recorded for `exact_kind::KindError` and
  `exact_ratio::RatioError` — a pattern across this family: every error type
  gets a real `Display` impl, but nothing in the current workspace actually
  formats an error message through it; every consumer matches on the enum
  variant instead.
- **`exact_arith`'s own facade test never calls into this crate at all** —
  it re-exports all 4 public items (`exact_arith/src/lib.rs:118`) but exercises none of
  them, the same bypass pattern independently found in `exact_add` and
  `exact_parse`'s catalogs.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_fmt/docs/item -name '*.md' -not -name readme.md | wc -l
# → 12
```
