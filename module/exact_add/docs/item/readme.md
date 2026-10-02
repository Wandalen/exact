# Item Entity

Catalog of every Rust Item declared in `exact_add`'s own source tree — 10
instances across 2 Item Kinds, all in `src/lib.rs` (this crate's only source
file). One file per declaration, classified by the closed Item Kind taxonomy
(`item_des.rulebook.md` OT001/OT002). Each instance records where the Item is
declared and, grep-verified against the 2 crates that depend on `exact_add`
directly (`exact_conserve`, `exact_arith` — per their own `Cargo.toml`),
every file and crate that uses it. The 9 Functions additionally carry their
Caller/Callee Tree closures.

`exact_add` is Tier 2, depending on `exact_kind` (the conserved value types)
and `exact_sign` (classifying which direction a saturating operation clamps
toward). It is a pure dispatch layer: every function is a one-line call into
`exact_kind`'s or `exact_sign`'s own methods/functions, with zero
intra-crate call edges — no function here calls another function declared in
this same crate.

## Disclosed deviations from the preferred design (module doc comment)

No local `AddError` exists. The preferred design lists `AddError{Overflow,
ScaleMismatch, NegNotAllowed}`; `Overflow` already exists as
`exact_kind::KindError::Overflow`, `ScaleMismatch` is unreachable under this
family's const-generic representation (two different `SCALE` values are two
different Rust types, caught at compile time), and `NegNotAllowed` has no
reachable call site since the only negation this crate exposes
([`money_checked_neg`](function/007_money_checked_neg.md)) is over a kind
that is already signed. Every fallible function here returns
`exact_kind::KindError` directly instead.

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
  - [ ] Is every citation line number grep-verified rather than recalled from an earlier read (OT008)?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Function | `function/` | 9 |
| **Total** | | **10** |

13 of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Struct, Enum, Union, Constant, Static, Trait, Implementation,
External Block, Macro Definition/Invocation. `exact_add` declares no local
error enum at all (see Disclosed deviations above), which is why it has
neither an Enum nor any Associated Item Kind.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{KindError, Money, Price, Quantity} | Use Declaration | 🔄 |
| function/001 | money_add | Function | 🔄 |
| function/002 | money_sub | Function | 🔄 |
| function/003 | qty_add | Function | 🔄 |
| function/004 | qty_sub | Function | 🔄 |
| function/005 | price_add | Function | 🔄 |
| function/006 | price_sub | Function | 🔄 |
| function/007 | money_checked_neg | Function | 🔄 |
| function/008 | money_saturating_add | Function | 🔄 |
| function/009 | qty_saturating_add | Function | 🔄 |

### Notable Findings

- **Only 2 of 9 functions have a real production caller outside `exact_add`
  itself**: [`money_add`](function/001_money_add.md) and
  [`qty_add`](function/003_qty_add.md), both called once each from
  `exact_conserve`'s `money_conserve_into`/`qty_conserve_into`. The other 7 —
  every subtraction, both price functions, the negation, and both saturating
  variants — are re-exported by `exact_arith` but have zero callers anywhere
  outside their own unit tests.
- **The facade's own end-to-end test never reaches for this crate's
  functions at all.** `exact_arith/tests/facade_test.rs`'s "a settlement
  runs end-to-end through the facade alone" test calls `Money`/`Quantity`
  methods (`checked_add`, `checked_sub`) directly rather than any of
  `exact_add`'s re-exported free functions — confirmed by grep across that
  test file. The facade re-exports all 9 names (`src/lib.rs:85-93`) and a
  separate re-export-resolution test touches a representative name from
  every other tier, but none of `exact_add`'s.
- **Zero intra-crate call edges.** Every one of the 9 functions is a single
  expression dispatching into `exact_kind` (8 of them) or additionally into
  `exact_sign` (1 of them, `money_saturating_add`) — no function in this
  crate calls another function in this crate, which is why every Caller Tree
  is either empty or a single external leaf.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_add/docs/item -name '*.md' -not -name readme.md | wc -l
# → 10
```
