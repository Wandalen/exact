# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_kind`'s own
source tree — 45 instances across 7 Item Kinds, all in `src/lib.rs` (this
crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against the 10 crates
that depend on `exact_kind` directly (`exact_add`, `exact_arith`,
`exact_bytes`, `exact_cmp`, `exact_conserve`, `exact_dust`, `exact_fmt`,
`exact_parse`, `exact_ratio`, `exact_snap` — per their own `Cargo.toml`),
every file and crate that uses it. Callable Kinds (Associated Function/
Method) additionally carry their Caller/Callee Tree closures.

`exact_kind` is the merge point of the family's migration: `exact_decimal`'s
signed `Decimal< const SCALE >` and `exact_qty`'s non-negative `Qty< const
SCALE >` both live here now, behaviourally unchanged, with `Money`/`Price`
aliasing the former and `Quantity` aliasing the latter. This makes it the
single largest catalog in the family — nearly the sum of `exact_decimal`'s 29
and `exact_qty`'s 26 instances — and the crate where identifier collisions
are sharpest: `Decimal` and `Qty` each declare `minor`, `whole`, `from_minor`,
`from_int`, `checked_add`, `checked_sub`, `checked_mul_int`, and `parse`
under the *same* names on two *different* types in the *same* file.
Associated Function/Method filenames disambiguate with an explicit
`_decimal`/`_qty` suffix throughout.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item (fn, struct, enum, trait, impl, etc.) declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind. No existing standard type covers per-declaration cataloging with exhaustive call-graph and cross-crate usage evidence.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale and invariants; `docs/item/` documents where a raw Rust Item is declared and every place it is used, regardless of whether it embodies a Domain Type.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Associated Function/Method kind additionally requires Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the actual workspace, not assumed (OT012)?
  - [ ] Do Associated Function/Method instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Are same-name methods on `Decimal` and `Qty` disambiguated by filename and cross-reference, never conflated?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 3 |
| Type Alias | `type_alias/` | 3 |
| Enum | `enum/` | 1 |
| Struct | `struct/` | 2 |
| Implementation | `implementation/` | 6 |
| Associated Constant | `associated_constant/` | 8 |
| Associated Function/Method | `associated_function/` | 22 |
| **Total** | | **45** |

Eight of the 15 taxonomy Kinds are absent from this crate: Module, Extern
Crate Declaration, Function, Union, Static, Trait, External Block, Macro
Definition/Invocation. Each was checked for systematically against
`src/lib.rs` (the crate's only source file) — none declares any instance of
these.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_minor::Backing | Use Declaration | 🔄 |
| use_declaration/002 | use exact_scale::{..} | Use Declaration | 🔄 |
| use_declaration/003 | use core::fmt | Use Declaration | 🔄 |
| type_alias/001 | Money | Type Alias | 🔄 |
| type_alias/002 | Price | Type Alias | 🔄 |
| type_alias/003 | Quantity | Type Alias | 🔄 |
| enum/001 | KindError | Enum | 🔄 |
| struct/001 | Decimal | Struct | 🔄 |
| struct/002 | Qty | Struct | 🔄 |
| implementation/001 | Display for KindError | Implementation | 🔄 |
| implementation/002 | Error for KindError | Implementation | 🔄 |
| implementation/003 | Decimal inherent impl | Implementation | 🔄 |
| implementation/004 | Display for Decimal | Implementation | 🔄 |
| implementation/005 | Qty inherent impl | Implementation | 🔄 |
| implementation/006 | Display for Qty | Implementation | 🔄 |
| associated_constant/001 | Decimal::ONE_MINOR | Associated Constant | 🔄 |
| associated_constant/002 | Decimal::ZERO | Associated Constant | 🔄 |
| associated_constant/003 | Decimal::EPSILON | Associated Constant | 🔄 |
| associated_constant/004 | Decimal::MAX | Associated Constant | 🔄 |
| associated_constant/005 | Decimal::MIN | Associated Constant | 🔄 |
| associated_constant/006 | Qty::ZERO | Associated Constant | 🔄 |
| associated_constant/007 | Qty::EPSILON | Associated Constant | 🔄 |
| associated_constant/008 | Qty::MAX | Associated Constant | 🔄 |
| associated_function/001 | Decimal::from_minor | Associated Function/Method | 🔄 |
| associated_function/002 | Decimal::from_int | Associated Function/Method | 🔄 |
| associated_function/003 | Decimal::minor | Associated Function/Method | 🔄 |
| associated_function/004 | Decimal::whole | Associated Function/Method | 🔄 |
| associated_function/005 | Decimal::checked_add | Associated Function/Method | 🔄 |
| associated_function/006 | Decimal::checked_sub | Associated Function/Method | 🔄 |
| associated_function/007 | Decimal::checked_mul_int | Associated Function/Method | 🔄 |
| associated_function/008 | Decimal::checked_neg | Associated Function/Method | 🔄 |
| associated_function/009 | Decimal::parse | Associated Function/Method | 🔄 |
| associated_function/010 | Qty::from_decimal | Associated Function/Method | 🔄 |
| associated_function/011 | Qty::from_minor | Associated Function/Method | 🔄 |
| associated_function/012 | Qty::from_int | Associated Function/Method | 🔄 |
| associated_function/013 | Qty::as_decimal | Associated Function/Method | 🔄 |
| associated_function/014 | Qty::minor | Associated Function/Method | 🔄 |
| associated_function/015 | Qty::whole | Associated Function/Method | 🔄 |
| associated_function/016 | Qty::checked_add | Associated Function/Method | 🔄 |
| associated_function/017 | Qty::checked_sub | Associated Function/Method | 🔄 |
| associated_function/018 | Qty::checked_mul_int | Associated Function/Method | 🔄 |
| associated_function/019 | Qty::parse | Associated Function/Method | 🔄 |
| associated_function/020 | Display::fmt for KindError | Associated Function/Method | 🔄 |
| associated_function/021 | Display::fmt for Decimal | Associated Function/Method | 🔄 |
| associated_function/022 | Display::fmt for Qty | Associated Function/Method | 🔄 |

### Notable Findings

- **Two true choke points**: [Decimal::from_minor](associated_function/001_from_minor_decimal.md) and [Qty::from_decimal](associated_function/010_from_decimal_qty.md) are each called by every other constructor/operation on their respective type — the range gate and the non-negativity gate, respectively.
- **Honest empty Caller Trees**: `Decimal::from_int`, `Decimal::checked_mul_int`, `Decimal::whole`, `Qty::from_int`, `Qty::as_decimal`, `Qty::whole`, `Qty::checked_mul_int`, and `Display::fmt for KindError` have zero callers anywhere outside `exact_kind`'s own tests (several have zero callers at all) — real gaps, not omissions, each individually grep-verified.
- **`KindError`'s `Display` is never rendered anywhere** in the current workspace — confirmed by grep. Every downstream crate that touches `KindError` reconstructs its own local error type via `match`, never by formatting the message.
- **`Price` is exercised identically to `Money`** everywhere it appears — no call site anywhere gives it behaviour `Money` lacks, the concrete evidence behind the crate's own disclosed "same type today" deviation.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_kind/docs/item -name '*.md' -not -name readme.md | wc -l
# → 45
```
