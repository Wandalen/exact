# Item Entity

Catalog of every Rust Item and Associated Item declared in `exact_bytes`'s
own source tree — 25 instances across 8 Item Kinds, all in `src/lib.rs`
(this crate's only source file). One file per declaration, classified by the
closed Item Kind taxonomy (`item_des.rulebook.md` OT001/OT002). Each instance
records where the Item is declared and, grep-verified against the one crate
that depends on `exact_bytes` directly (`exact_arith`, per its own
`Cargo.toml`), every file and crate that uses it. Callable Kinds (Function,
Associated Function/Method) additionally carry their Caller/Callee Tree
closures.

`exact_bytes` is Tier 2, depending on `exact_kind` and `exact_scale`.
Net-new: no real precedent exists in the 5 real crates — the closest is the
real codebase's own unimplemented `format/001_transaction_log_encoding.md`
spec for a bare 8-byte `amount` field. This crate widens that to a
self-describing 10-byte `Wire` record (minor, scale, kind), since a bare
amount cannot be decoded back into a specific kind without an external
convention recording which kind and scale it was written at (module doc
comment, `src/lib.rs:7-13`).

Two private top-level functions, `kind_error_to_wire_error` (`src/lib.rs:92`)
and `check_header` (`src/lib.rs:104`),
is excluded from this catalog per `item_des.rulebook.md`'s own Caller Tree
Content rule (private/`pub(crate)`/`pub(super)` helpers get no Item Instance
of their own) — it still appears as a real hop, cited as a plain
`src/lib.rs:92`, in the 3 `*_from_wire` functions' Callee Trees.

### Type Declaration

- **Decision Criteria**: Use `docs/item/` to catalog every Rust Item (fn, struct, enum, trait, impl, etc.) declared in this crate's own source tree — one Item Instance per declared Item, classified by its exact Item Kind. No existing standard type covers per-declaration cataloging with exhaustive call-graph and cross-crate usage evidence.
- **Contrast with `docs/type/`**: `docs/type/` documents a Domain Type's design rationale and invariants; `docs/item/` documents where a raw Rust Item is declared and every place it is used, regardless of whether it embodies a Domain Type.
- **Required Sections**: Representation, Kind, Definition, File Usage, Crate Usage
  (Function and Associated Function/Method kinds additionally require Caller Tree, Callee Tree)
- **Overview Table Columns**: `ID`, `Name`, `Kind`, `Status`
- **Quality Checklist**:
  - [ ] Does every instance declare exactly one Kind from the closed 15+3 taxonomy?
  - [ ] Are File Usage and Crate Usage exhaustively grep-verified against the actual workspace, not assumed (OT012)?
  - [ ] Do Function/Associated Function instances carry both Caller Tree and Callee Tree (OT006)?
  - [ ] Are the two private helper functions correctly excluded from its own instance while still appearing as a real Callee Tree hop?
  - [ ] Was a same-text, unrelated identifier (e.g. a string literal) correctly excluded as a collision rather than counted as usage (OT012)?

### Kind Distribution

| Kind | Directory | Instances |
|------|-----------|-----------|
| Use Declaration | `use_declaration/` | 1 |
| Constant | `constant/` | 4 |
| Enum | `enum/` | 1 |
| Struct | `struct/` | 1 |
| Implementation | `implementation/` | 3 |
| Associated Constant | `associated_constant/` | 1 |
| Associated Function/Method | `associated_function/` | 7 |
| Function | `function/` | 6 |
| **Total** | | **24** |

Seven of the 15 taxonomy Kinds are absent: Module, Extern Crate Declaration,
Type Alias, Union, Static, Trait, External Block, Macro Definition/Invocation
(8, not 7 — Associated Type is the 3rd Associated Item Kind and is also
absent, since `Wire`'s impl declares no associated type).

### Overview Table

| ID | Name | Kind | Status |
|----|------|------|--------|
| use_declaration/001 | use exact_kind::{ KindError, Money, Price, Quantity } | Use Declaration | 🔄 |
| constant/001 | KIND_MONEY | Constant | 🔄 |
| constant/002 | KIND_QTY | Constant | 🔄 |
| constant/003 | KIND_PRICE | Constant | 🔄 |
| constant/004 | const _ (MONEY_SCALE fits u8 assertion) | Constant | 🔄 |
| constant/005 | SCALE_BYTE | Constant | 🔄 |
| enum/001 | WireError | Enum | 🔄 |
| struct/001 | Wire | Struct | 🔄 |
| implementation/001 | Display for WireError | Implementation | 🔄 |
| implementation/002 | Error for WireError | Implementation | 🔄 |
| implementation/003 | Wire inherent impl | Implementation | 🔄 |
| associated_constant/001 | Wire::ENCODED_LEN | Associated Constant | 🔄 |
| associated_function/001 | Display::fmt for WireError | Associated Function/Method | 🔄 |
| associated_function/002 | Wire::new | Associated Function/Method | 🔄 |
| associated_function/003 | Wire::minor | Associated Function/Method | 🔄 |
| associated_function/004 | Wire::scale | Associated Function/Method | 🔄 |
| associated_function/005 | Wire::kind | Associated Function/Method | 🔄 |
| associated_function/006 | Wire::to_bytes | Associated Function/Method | 🔄 |
| associated_function/007 | Wire::from_bytes | Associated Function/Method | 🔄 |
| function/001 | money_to_wire | Function | 🔄 |
| function/002 | money_from_wire | Function | 🔄 |
| function/003 | qty_to_wire | Function | 🔄 |
| function/004 | qty_from_wire | Function | 🔄 |
| function/005 | price_to_wire | Function | 🔄 |
| function/006 | price_from_wire | Function | 🔄 |

### Notable Findings

- **Two of `Wire`'s three field accessors are never called anywhere, not even by this crate's own tests.** [`Wire::minor`](associated_function/003_wire_minor.md) and [`Wire::scale`](associated_function/004_wire_scale.md) are both fully honest-empty: every real consumer, including `check_header` and the 3 `*_from_wire` functions, reads the private field directly instead (same-module privilege), and the test suite only asserts `.kind()` or the round-tripped value as a whole. [`Wire::kind`](associated_function/005_wire_kind.md) is the one accessor actually exercised — by tests only, never in production.
- **Only the Money roundtrip is exercised at the facade level.** `exact_arith/tests/facade_test.rs` constructs and decodes a `Wire` via `money_to_wire`/`money_from_wire` in its own end-to-end settlement test — but never calls `qty_to_wire`/`qty_from_wire`/`price_to_wire`/`price_from_wire`, which the facade only re-exports. This is the opposite pattern from most sibling crates (where the facade test bypasses the re-exported functions entirely); here it partially exercises them.
- **`WireError`'s `Display` is never rendered anywhere**, mirroring the identical finding already recorded for `exact_kind::KindError` and `exact_ratio::RatioError` — every real use of the error type is construction or pattern matching, never formatting.
- **A genuine identifier collision was found and excluded**: `substrate/demiurg/demiurg_schema/tests/match_test.rs` uses the string literal `"Wire"` as an unrelated component name, with no dependency on `exact_bytes` — confirmed via `Cargo.toml` and excluded per OT012's collision-disambiguation rule, not counted as usage.

### Regenerate

```bash
# Confirm instance-file count matches this readme's Overview Table row count
find module/exact_bytes/docs/item -name '*.md' -not -name readme.md | wc -l
# → 25
```
