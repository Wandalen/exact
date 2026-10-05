# Format: Wire Record Encoding

### Scope

- **Purpose**: Fix the byte layout `Wire` reads and writes, so a decoder written against this document alone reconstructs the same three fields a `Wire` encoder produced, with no shared external convention required.
- **Responsibility**: `Wire`'s field order, widths, byte order, and what a round trip guarantees.
- **In Scope**: The 10-byte record `Wire::to_bytes`/`Wire::from_bytes` encode and decode.
- **Out of Scope**: How a decoded `Wire` becomes a specific kind, or is refused (→ [Wire Error Adds Overflow And Negative](../decisions/001_wire_error_overflow_and_negative_variants.md)); the conserved value types a `Wire` round-trips (→ [`exact_kind`](../../../exact_kind/readme.md)); the scale constant a wire record is checked against (→ [`exact_scale`](../../../exact_scale/readme.md)).

### Why Self-Describing, Not A Bare Amount

The closest real precedent — this family's own retired
`format/001_transaction_log_encoding.md` — specified an unimplemented 8-byte
`amount` field: a bare integer count of minor units, with no kind or scale
recorded alongside it. A bare amount cannot be decoded back into a specific
kind without an external convention — a side channel, a file-level header,
or an assumption baked into the reader — recording which kind and which
scale it was written at. `Wire` widens the 8-byte amount to a 10-byte
record carrying that context in the record itself: a decoded value tells its
own reader everything `money_from_wire`/`qty_from_wire`/`price_from_wire`
need to validate it, rather than trusting the reader's own bookkeeping to
still agree with whatever wrote it.

### Encoding Structure

All multi-byte fields are **little-endian**. `Wire::ENCODED_LEN` is `10`
bytes, fixed — every record is the same size regardless of the value it
carries, so there is no length prefix and no varint.

| Offset | Width | Field | Value |
|-------:|------:|-------|-------|
| 0 | 8 | minor | The value's minor-unit count, as a signed `i64` |
| 8 | 1 | scale | The scale the value was written at — `exact_scale::MONEY_SCALE` for every kind this crate encodes today |
| 9 | 1 | kind | A discriminator: [`KIND_MONEY`] (`0`), [`KIND_QTY`] (`1`), or [`KIND_PRICE`] (`2`) |

`minor` carries no range check of its own at the `Wire` level — `Wire::new`
is infallible and stores exactly what it is given. The check against a
kind's declared ceiling (and, for `Quantity`, against negative values)
happens in `money_from_wire`/`qty_from_wire`/`price_from_wire`, after
decoding, because that is the only point where which kind is being decoded
into — and therefore which range applies — is known.

### Round-Trip Guarantee

Encoding and decoding through `Wire` is lossless for any value a kind's own
constructor accepted: `kind_from_wire(kind_to_wire(v)) == Ok(v)` for every
`v` of that kind, including a negative `Money` — a `Wire`'s `minor` field is
signed, so a debit encodes and decodes exactly like a credit. The guarantee
holds through raw bytes as well as through the `Wire` struct itself:
`Wire::from_bytes(&Wire::to_bytes(w)) == Ok(w)` for every `Wire` built from a
valid kind value.

Decoding checks `kind` and `scale` before trusting `minor`: a record encoded
as one kind is refused by another kind's decoder (`WireError::BadKind`), and
a `scale` byte that does not match the kind's expected scale is refused
(`WireError::BadScale`) even when `kind` is correct — the two checks are
independent, and either alone is enough to refuse a record. A byte slice
shorter than `Wire::ENCODED_LEN` is refused as `WireError::Truncated` rather
than zero-padded.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:41-48` | `KIND_MONEY`, `KIND_QTY`, `KIND_PRICE`, and the compile-time check that `MONEY_SCALE` fits the `scale` byte |
| `src/lib.rs:97-172` | `Wire`'s struct definition, `ENCODED_LEN`, `new`, `to_bytes`, `from_bytes` |
| `src/lib.rs:174-252` | The six `*_to_wire`/`*_from_wire` functions, one pair per kind |

### Tests

| File | Relationship |
|------|--------------|
| `tests/wire_roundtrip_test.rs` | Round-tripping per kind, through both the `Wire` struct and raw bytes; `BadKind`, `BadScale`, and `Truncated` refusals |
