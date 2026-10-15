# Invariant: Buffer Writes Stay In Bounds

### Scope

- **Purpose**: State that `fmt_into` can never write past the end of a caller-supplied buffer, so a caller can pass a buffer of any size, including one too small for the rendered text, without risking a panic or out-of-bounds write.
- **Responsibility**: `ByteBufWriter::write_str`'s capacity check.
- **In Scope**: Whether a write can exceed `buf`'s length.
- **Out of Scope**: What `buf`'s contents are after a failed write, when a multi-piece `Display` impl has already written some pieces successfully before a later one overflows — a content question, not a bounds-safety one (→ `../algorithm/001_decimal_rendering_per_kind.md`'s "Buffer-Writing Primitive" section, which documents that case in full); whether rendering itself can fail — it cannot, for this crate's three kinds (→ the same algorithm doc).

### Statement

`ByteBufWriter::write_str` (`src/lib.rs:70-83`) compares the incoming
fragment's length against the buffer's remaining capacity — `self.len +
bytes.len() > self.buf.len()` — before copying anything. If the fragment
would not fit, it copies nothing and returns `Err(core::fmt::Error)`
immediately, which `fmt_into` maps to `FmtError::BufFull`. No code path in
this crate indexes into `buf` past a length already confirmed to fit. A
buffer of any size, including zero, is therefore safe to pass to `fmt_into` —
the result is either a successful render or a reported `BufFull`, never a
panic or a write outside `buf`'s bounds.

### Rationale

`fmt_into` exists specifically for a caller that owns a fixed-size buffer —
often a reused scratch buffer on a hot path — and wants to avoid the
heap allocation `to_string()` requires. That use case only works if an
undersized buffer is a reportable error rather than undefined behavior: a
caller sizing a buffer from a worst-case estimate needs the failure mode for
an underestimate to be a `Result`, not a panic that takes down the hot path
it was trying to keep allocation-free. Checking capacity before copying,
rather than copying and discovering the overrun after the fact, is what
makes that guarantee hold unconditionally rather than only for inputs the
caller happened to size correctly.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:64-68` | `ByteBufWriter` — the private buffer-backed writer |
| `src/lib.rs:70-83` | `write_str` — the bounds check, before any copy |
| `src/lib.rs:95-101` | `fmt_into` — maps a bounds failure to `FmtError::BufFull` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/fmt_test.rs` | `a_buffer_exactly_the_rendered_length_succeeds` and the too-small-buffer refusal case |
