# Item Entity

Catalog of every Rust Item declared in `exact_arith`'s own source tree — 14
instances, all Use Declarations, all in `src/lib.rs` (this crate's only
source file). One file per `pub use` block, classified by the closed Item
Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance records
where the re-export is declared and every file and crate that touches the
names it carries, verified via a **full-workspace grep**, not just this
crate's own test suite or its direct `Cargo.toml` dependents — the same
discipline `exact_conserve`'s catalog established after its own first-draft
miss (see that crate's readme Notable Findings).

`exact_arith` is Tier 4, the facade over all 14 other new crates in this
family. It declares no type, function, struct, enum, trait, or impl of its
own — its mechanical purity is enforced by its own ported source-grep test
(`tests/facade_test.rs::the_facade_source_is_re_exports_and_documentation_only`),
which this catalog's OT006 scoping reflects directly: zero Function or
Associated Function/Method kinds exist here, so zero files in this crate
carry a Caller Tree or Callee Tree section — a Use Declaration has neither.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale; `docs/item/` documents where a raw Rust Item is declared and every place it is used.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree — none apply in this crate)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the **full workspace**, not only direct `Cargo.toml` dependents or this crate's own tests (OT012)?
  - [ ] Is every `smoke_*`-prefixed consumer labeled **Demo-lane**, never **Production** — checked against that crate's own module doc comment, not assumed from the name alone (a real miscategorization this catalog caught and fixed in its own first draft, see Notable Findings)?
  - [ ] Where a re-export block has names with no confirmed caller, is that stated as an explicit finding rather than left implicit in an all-or-nothing summary?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 14 |
| **Total** | | **14** |

All 14 other taxonomy Kinds are absent: Module, Extern Crate Declaration,
Function, Type Alias, Struct, Enum, Union, Constant, Static, Trait,
Implementation, External Block, Macro Definition, Macro Invocation. No
Associated Item Kind either (Associated Function/Method, Associated
Constant, Associated Type) — a pure facade has nothing to attach one to.

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | pub use exact_minor::{ ... } | Use Declaration | 🔄 |
| use_declaration/002 | pub use exact_scale::{ ... } | Use Declaration | 🔄 |
| use_declaration/003 | pub use exact_round::{ ... } | Use Declaration | 🔄 |
| use_declaration/004 | pub use exact_sign::{ ... } | Use Declaration | 🔄 |
| use_declaration/005 | pub use exact_kind::{ ... } | Use Declaration | 🔄 |
| use_declaration/006 | pub use exact_add::{ ... } | Use Declaration | 🔄 |
| use_declaration/007 | pub use exact_ratio::{ ... } | Use Declaration | 🔄 |
| use_declaration/008 | pub use exact_parse::{ ... } | Use Declaration | 🔄 |
| use_declaration/009 | pub use exact_fmt::{ ... } | Use Declaration | 🔄 |
| use_declaration/010 | pub use exact_bytes::{ ... } | Use Declaration | 🔄 |
| use_declaration/011 | pub use exact_snap::{ ... } | Use Declaration | 🔄 |
| use_declaration/012 | pub use exact_cmp::{ ... } | Use Declaration | 🔄 |
| use_declaration/013 | pub use exact_dust::{ ... } | Use Declaration | 🔄 |
| use_declaration/014 | pub use exact_conserve::{ ... } | Use Declaration | 🔄 |

### Notable Findings

- **A 3-way usage split across the 14 re-export blocks, verified via
  full-workspace grep rather than assumed from this crate's own test suite
  alone:**
  - **Touched by this facade's own doc-test/test suite** (7 of 14):
    `exact_scale`, `exact_round`, `exact_sign`, `exact_kind`, `exact_bytes`,
    `exact_dust`, `exact_conserve` — though each still carries at least one
    re-exported name with no confirmed caller anywhere (e.g. `exact_scale`'s
    `HEADROOM_FACTOR`, `exact_kind`'s bare `Decimal`/`Qty`), so "touched"
    here means partial, not exhaustive, exercise.
  - **Downstream-only, zero facade-test touch** (1 of 14): `exact_minor` —
    real production usage in `exchange_core`/`exchange_types`, yet not one
    of its 9 names is imported by this crate's own `tests/facade_test.rs`.
    The inverse of the usual pattern, where a name untested at the facade
    layer is also unused everywhere else.
  - **Touched nowhere through this facade** (6 of 14): `exact_add`,
    `exact_ratio`, `exact_parse`, `exact_fmt`, `exact_snap`, `exact_cmp` —
    entirely dormant from this facade's perspective, confirmed via
    full-workspace grep with every positive-looking hit (a `Term::Ratio` in
    unrelated `module/play_verify`, `Tick`/`Lot` as simulation/session
    concepts in `cluster_macro`/`session_ledger`) checked against the
    hitting crate's own `Cargo.toml` and dismissed as a name collision, not
    a real caller.
- **`exact_kind` and `exact_conserve` are the two blocks with genuinely
  extensive real production reach** — `exact_kind`'s `Money`/`Quantity`/
  `KindError` are used across nearly every `substrate/exchange/` and
  `module/division/010_economy_market` crate; `exact_conserve`'s `Entry`/
  `Report`/`verify`/`ConservationError` gate `cluster_economy`'s settlement
  path directly (`src/market.rs:507-508,518-519`) and are re-exported one
  hop further by `exchange_core`. Both facts were independently established
  in each leaf crate's own catalog and re-confirmed here against the
  facade's re-export specifically.
- **This catalog's own first draft mislabeled two `smoke_*`-prefixed
  consumers as Production** (`smoke_cluster_economy_market`,
  `smoke_module_cluster_integration`, in `use_declaration/005`) before their
  own module doc comments ("Headless smoke lane grading...") were checked
  directly — corrected to **Demo-lane**, matching the project-wide
  convention already established for `smoke_exact_market_split` and
  `smoke_exchange_core`. A second, milder version of the same slip (a
  one-off "Production-adjacent (demo lane)" phrasing in
  `use_declaration/003`, and a missed row in `use_declaration/013`) was
  caught in the same pass and normalized to the single canonical term — the
  label follows what a crate's own source says it is, never its name alone.
- **The facade's own mechanical self-checks are exercised, not just
  asserted**: `the_facade_source_is_re_exports_and_documentation_only`
  greps this crate's own compiled-in source for 9 forbidden keywords
  (`fn `, `struct `, `enum `, `trait `, `impl `, `const `, `static `,
  `type `, `macro_rules`) every test run, and
  `the_backing_width_is_declared_in_exactly_one_place` confirms `Backing`'s
  declaration exists only in `exact_minor`, never redeclared in `exact_kind`
  or here — both are the facade-level evidence that this crate is, and
  stays, pure re-export.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_arith/docs/item -name '*.md' -not -name readme.md | wc -l
# → 14
```
