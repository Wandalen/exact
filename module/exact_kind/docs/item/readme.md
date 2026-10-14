# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_kind`'s own
source tree — 65 instances across 8 Item Kinds, all in `src/lib.rs` (this
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
SCALE >` both live here now, behaviourally unchanged, with `Money` aliasing the
former, `Quantity` aliasing the latter, and `Price` aliasing `PriceAt< SCALE >`.
All three are aliases of one struct, `Amount< K, SCALE >`, one per kind
(→ [ADR-006](../decisions/006_one_amount_struct_generic_over_kind.md)). This makes it the
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
| Use Declaration | `use_declaration/` | 5 |
| Type Alias | `type_alias/` | 6 |
| Enum | `enum/` | 1 |
| Struct | `struct/` | 4 |
| Trait | `trait/` | 1 |
| Implementation | `implementation/` | 11 |
| Associated Constant | `associated_constant/` | 8 |
| Associated Function/Method | `associated_function/` | 29 |
| **Total** | | **65** |

Six of the 15 taxonomy Kinds have no public instance in this crate: Module,
Extern Crate Declaration, Function, Union, Static, External Block. Each was
checked for systematically against `src/lib.rs` (the crate's only source
file). Private items get no page, as for every crate in the family: the
`sealed` module and its `Sealed` trait, the private functions
(`kind_overflow`, `render`, `fraction_digits`), `Amount`'s private
`value()`/`from_value()`, and the three `Sealed` impls are listed on the
public page that uses them — [Kind](trait/001_kind.md) and
[Amount](struct/001_amount.md). The three `impl Kind` are empty and are
listed on [Kind](trait/001_kind.md). `+` and `-` are four generic impls on
`Amount`, not a macro, so there is no Macro Definition or Macro Invocation
([ADR-006](../decisions/006_one_amount_struct_generic_over_kind.md)).

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_minor::{ Backing, Minor, … } | Use Declaration | 🔄 |
| use_declaration/002 | use exact_scale::{..} | Use Declaration | 🔄 |
| use_declaration/003 | use core::fmt | Use Declaration | 🔄 |
| use_declaration/004 | use exact_round::{ RoundError, Rounding, round_div } | Use Declaration | 🔄 |
| use_declaration/005 | use core::marker::PhantomData | Use Declaration | 🔄 |
| type_alias/001 | Money | Type Alias | 🔄 |
| type_alias/002 | Price | Type Alias | 🔄 |
| type_alias/003 | Quantity | Type Alias | 🔄 |
| type_alias/004 | Decimal | Type Alias | 🔄 |
| type_alias/005 | Qty | Type Alias | 🔄 |
| type_alias/006 | PriceAt | Type Alias | 🔄 |
| enum/001 | KindError | Enum | 🔄 |
| struct/001 | Amount | Struct | 🔄 |
| struct/002 | MoneyKind | Struct | 🔄 |
| struct/003 | QtyKind | Struct | 🔄 |
| struct/004 | PriceKind | Struct | 🔄 |
| trait/001 | Kind | Trait | 🔄 |
| implementation/001 | Display for KindError | Implementation | 🔄 |
| implementation/002 | Error for KindError | Implementation | 🔄 |
| implementation/003 | Decimal inherent impl | Implementation | 🔄 |
| implementation/004 | Display for Decimal | Implementation | 🔄 |
| implementation/005 | Qty inherent impl | Implementation | 🔄 |
| implementation/006 | Display for Qty | Implementation | 🔄 |
| implementation/007 | Debug for Amount | Implementation | 🔄 |
| implementation/008 | Add for Amount | Implementation | 🔄 |
| implementation/009 | Add< Amount > for Result | Implementation | 🔄 |
| implementation/010 | Sub for Amount | Implementation | 🔄 |
| implementation/011 | Sub< Amount > for Result | Implementation | 🔄 |
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
| associated_function/023 | Decimal::rescale | Associated Function/Method | 🔄 |
| associated_function/024 | Qty::rescale | Associated Function/Method | 🔄 |
| associated_function/025 | Debug::fmt for Amount | Associated Function/Method | 🔄 |
| associated_function/026 | Add::add for Amount | Associated Function/Method | 🔄 |
| associated_function/027 | Add::add for Result | Associated Function/Method | 🔄 |
| associated_function/028 | Sub::sub for Amount | Associated Function/Method | 🔄 |
| associated_function/029 | Sub::sub for Result | Associated Function/Method | 🔄 |

### Notable Findings

- **Two true choke points**: [Decimal::from_minor](associated_function/001_from_minor_decimal.md) and [Qty::from_decimal](associated_function/010_from_decimal_qty.md) are each called by every other constructor/operation on their respective type — the range gate and the non-negativity gate, respectively.
- **Honest empty Caller Trees**: `Decimal::from_int`, `Decimal::checked_mul_int`, `Decimal::whole`, `Qty::from_int`, `Qty::as_decimal`, `Qty::whole`, `Qty::checked_mul_int`, and `Display::fmt for KindError` have zero callers anywhere outside `exact_kind`'s own tests (several have zero callers at all) — real gaps, not omissions, each individually grep-verified.
- **`KindError`'s `Display` is never rendered anywhere** in the current workspace — confirmed by grep. Every downstream crate that touches `KindError` reconstructs its own local error type via `match`, never by formatting the message.
- **One struct, three kinds** — [Amount](struct/001_amount.md) holds one `Minor` and a zero-sized kind; [Decimal](type_alias/004_decimal.md), [Qty](type_alias/005_qty.md) and [PriceAt](type_alias/006_price_at.md) are its aliases, each with its own `impl` block, so each keeps its own API. [Kind](trait/001_kind.md) is sealed, so the set of kinds is closed ([ADR-006](../decisions/006_one_amount_struct_generic_over_kind.md)).
- **`PriceAt`'s members have no pages of their own** — each one only delegates to the matching `Decimal` member, so [its alias page](type_alias/006_price_at.md) lists them in one table. `Price` is its alias at the standard scale ([type_alias/002](type_alias/002_price.md)).
- **`+` and `-` return a `Result`, on all three kinds** — four generic impls on `Amount`: [`+`](implementation/008_add_for_amount.md), [`+` on a `Result`](implementation/009_add_for_result.md), [`-`](implementation/010_sub_for_amount.md) and [`-` on a `Result`](implementation/011_sub_for_result.md), each calling the kind's own `checked_add` or `checked_sub` through a private hook. The impls with a `Result` on the left let `( a + b + c )?` take one `?` ([ADR-005](../decisions/005_checked_operators_return_a_result.md)). The demo lane uses them; no production caller does yet.
- **`Debug` is written by hand** — [it prints the kind's own name](implementation/007_debug_for_amount.md), `Qty { minor: … }`, as each struct's derived `Debug` did, not `Amount { …, kind: PhantomData<…> }`.
- **`rescale` has no caller outside this crate yet** — [Decimal::rescale](associated_function/023_rescale_decimal.md) is reached only through `Qty::rescale`, `PriceAt::rescale` and this crate's own tests.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_kind/docs/item -name '*.md' -not -name readme.md | wc -l
# → 65
```
