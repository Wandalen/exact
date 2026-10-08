# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `KIND_MONEY` | const | `src/lib.rs:42` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `KIND_QTY` | const | `src/lib.rs:44` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `KIND_PRICE` | const | `src/lib.rs:46` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `WireError` | enum | `src/lib.rs:57` | [Wire Error Adds Overflow And Negative](../decisions/001_wire_error_overflow_and_negative_variants.md) |
| `WireError`'s `Display` impl | trait impl | `src/lib.rs:75` | [Wire Error Adds Overflow And Negative](../decisions/001_wire_error_overflow_and_negative_variants.md) |
| `Wire` | struct | `src/lib.rs:120` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::ENCODED_LEN` | assoc const | `src/lib.rs:131` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::new` | fn | `src/lib.rs:140` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::minor` | fn | `src/lib.rs:147` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::scale` | fn | `src/lib.rs:154` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::kind` | fn | `src/lib.rs:161` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::to_bytes` | fn | `src/lib.rs:168` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `Wire::from_bytes` | fn | `src/lib.rs:182` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `money_to_wire` | fn | `src/lib.rs:196` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `money_from_wire` | fn | `src/lib.rs:208` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `qty_to_wire` | fn | `src/lib.rs:216` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `qty_from_wire` | fn | `src/lib.rs:229` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `price_to_wire` | fn | `src/lib.rs:237` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |
| `price_from_wire` | fn | `src/lib.rs:247` | [Wire Record Encoding](../format/001_wire_record_encoding.md) |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
