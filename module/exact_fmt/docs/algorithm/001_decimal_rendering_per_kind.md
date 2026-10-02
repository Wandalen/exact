# Algorithm: Decimal Rendering Per Kind

### Scope

- **Purpose**: State exactly what `money_fmt`, `qty_fmt`, `price_fmt` and `fmt_into` produce for any value, so a caller can predict the rendered text — or the buffer-write outcome — without running it.
- **Responsibility**: The three per-kind `Display` wrappers, the allocation-free `fmt_into` buffer-writing primitive, and why both exist alongside the `Display` impls they render through.
- **In Scope**: Per-kind dispatch to `Display`, the buffer-writing mechanism and its one failure mode, and why `Display` itself is not implemented in this crate.
- **Out of Scope**: The grammar and the parse half of the round-trip (→ [`exact_parse`'s parsing algorithm](../../../exact_parse/docs/algorithm/001_decimal_parsing_per_kind.md)); `Decimal`/`Qty`'s own `Display` implementation and the round-trip guarantee's proof, owned by `exact_kind`.

### Why `Display` Is Not Implemented Here

The preferred design for this crate states `Display` as "only as a wrapper over `fmt_into` — no independent formatting logic," which would mean this crate owns the trait impl. That is impossible under Rust's orphan rules given the family's dependency direction: `exact_fmt` depends on `exact_kind`, so neither `core::fmt::Display` (a foreign trait) nor `exact_kind::Decimal`/`Qty` (a foreign type) is local to this crate, and the impl is refused outright. Reversing the dependency so `exact_kind` depended on `exact_fmt` instead was rejected as contradicting the family's own topological tier order for no behavioral gain.

`Display` therefore stays on `exact_kind::Decimal`/`Qty`, ported unchanged from the real codebase. This crate's three per-kind functions are a thin layer over that existing impl — the closest satisfiable reading of "no independent formatting logic" available under the constraint above.

### Per-Kind Rendering

| Function | Renders | Mechanism |
|----------|---------|-----------|
| `money_fmt` | `Money` (`exact_kind::Decimal<MONEY_SCALE>`) | `v.to_string()`, which calls `Decimal`'s `Display` impl |
| `qty_fmt` | `Quantity` (`exact_kind::Qty<MONEY_SCALE>`) | `v.to_string()`, which calls `Qty`'s `Display` impl — itself a pass-through to the inner `Decimal`'s |
| `price_fmt` | `Price` (`exact_kind::Decimal<MONEY_SCALE>`) | Identical to `money_fmt` — `Price` is `Money` under `exact_kind`'s disclosed deviation |

All three allocate a `String` and can never fail: `Display` for `Decimal`/`Qty` has no error path — trailing fractional zeros are trimmed, the sign is printed only when negative, and a zero-`SCALE` value prints no `.` at all.

### Buffer-Writing Primitive: `fmt_into`

`fmt_into(value: impl Display, buf: &mut [u8]) -> Result<usize, FmtError>` renders any `Display` value — not only this crate's three kinds — into a caller-provided byte buffer with no allocation, returning the byte count written. It exists alongside the `String`-returning functions above specifically to avoid the heap allocation `to_string()` requires, for a caller that already owns a buffer (for example a reused scratch buffer on a hot path) and wants the rendered text written directly into it.

Internally, `fmt_into` drives a private `ByteBufWriter` through `core::fmt::Write`, forwarding whatever `write!` calls the value's own `Display` impl makes — `Decimal`'s impl alone can emit up to three separate pieces (an optional `"-"`, the whole part, and an optional `.` plus trimmed fractional digits). Each `write_str` call `ByteBufWriter` receives is checked and copied atomically: it compares the incoming fragment's length against the buffer's remaining capacity before copying, and copies nothing for that fragment if it would overflow, returning `FmtError::BufFull` immediately. Because a multi-piece `Display` impl can have already written earlier pieces successfully before a later piece overflows, the buffer's contents on error are documented as unspecified rather than guaranteed-untouched — not every failure happens on the very first byte. A buffer exactly the rendered length succeeds, with no slack required (`tests/fmt_test.rs`'s `a_buffer_exactly_the_rendered_length_succeeds`).

### Round-Trip Property

`money_fmt`, `qty_fmt` and `price_fmt` are the exit half of the family's round-trip guarantee: `render(parse(s)) == s` for every canonical spelling `s`, and `parse(render(v)) == v` for every representable `v`. The guarantee itself is proved once, at the `exact_kind` layer both this crate and `exact_parse` dispatch to (`exact_kind/tests/parse_render_test.rs`) — see [`exact_parse`'s parsing algorithm doc](../../../exact_parse/docs/algorithm/001_decimal_parsing_per_kind.md) for the parse half and the shared citation.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:1-25` | Module doc — why `Display` cannot move into this crate, and what stays ported unchanged |
| `src/lib.rs:27` | Imports `Money`, `Price`, `Quantity` from `exact_kind` |
| `src/lib.rs:29-35` | `FmtError` — the one failure mode, `BufFull` |
| `src/lib.rs:37-46` | `Display` for `FmtError` |
| `src/lib.rs:48` | `core::error::Error` for `FmtError` |
| `src/lib.rs:50-54` | `ByteBufWriter` — the private buffer-backed `core::fmt::Write` target |
| `src/lib.rs:56-69` | `ByteBufWriter::write_str` — the atomic per-fragment capacity check |
| `src/lib.rs:71-87` | `fmt_into` — doc comment states the no-allocation rationale (lines 71-72, 74), implementation at 81-87 |
| `src/lib.rs:89-94` | `money_fmt` |
| `src/lib.rs:96-101` | `qty_fmt` |
| `src/lib.rs:103-108` | `price_fmt` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/fmt_test.rs` | Per-kind rendering matches `Display`, `fmt_into` buffer writes, the too-small-buffer refusal, and the exact-fit success |
| `../exact_kind/tests/parse_render_test.rs` | The `Display` impl and round-trip property this crate dispatches to (owned and proved by `exact_kind`, not re-tested here) |
