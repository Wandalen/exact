# 001: fmt_into

## Representation

Render any `Display`-able value into a caller-provided byte buffer, with no
allocation. The crate's one genuinely load-bearing export — every other
function here is a thin `to_string()` wrapper, but this one is the reason
the crate exists (the preferred design's own stated purpose: a non-allocating
render path).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_fmt/src/lib.rs:95-101`

```rust
pub fn fmt_into( value : impl core::fmt::Display, buf : &mut [ u8 ] ) -> Result< usize, FmtError >
{
  use core::fmt::Write;
  let mut writer = ByteBufWriter { buf, len : 0 };
  write!( writer, "{value}" ).map_err( | _ | FmtError::BufFull )?;
  Ok( writer.len )
}
```

Line 83's `use core::fmt::Write;` is a function-body-local `use` — not a
top-level Item (`item_des.rulebook.md` § Item Kind Taxonomy : Stable Item
Kinds: "every **top-level** Rust Item"), so it gets no Item Instance of its
own; it brings the `Write` trait's `write_fmt` method into scope for the
`write!` macro below, local to this function only.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 95-101 | Declaration |
| `tests/fmt_test.rs:20-26` | — | Exact round-trip into an exactly-sized buffer |
| `tests/fmt_test.rs:28-35` | — | `FmtError::BufFull` on a too-small buffer |
| `tests/fmt_test.rs:38-44` | — | Byte count returned matches the rendered text's length |
| `tests/fmt_test.rs:56` | — | Every kind, and a negative sign |
| `tests/fmt_test.rs:73` | — | `BufFull` when the buffer runs out partway, after an earlier piece fit |
| `tests/fmt_test.rs:96` | — | `BufFull` on a zero-length buffer |
| `exact_arith/src/lib.rs:118` | — | Facade re-export |
| `exact_arith/tests/no_alloc_test.rs:51,53-54,57` | — | Renders every kind into a stack buffer, checked to make no heap allocation |

No production call site anywhere outside this crate's own tests — an honest
empty finding. `exact_arith`'s `tests/no_alloc_test.rs` calls it through the
re-export to count allocations.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Exercised by all 3 of this crate's own tests |
| `exact_arith` | `src/lib.rs`, `tests/no_alloc_test.rs` | Re-export; its allocation test calls it |

## Caller Tree

- **External:** `exact_arith`'s own test, `no_alloc_test.rs:51,53-54,57` (test-context, via the re-exported name)

## Callee Tree

- [ByteBufWriter](../struct/001_byte_buf_writer.md) (`src/lib.rs:98`, constructed)
- [write_str for ByteBufWriter](../associated_function/002_write_str_for_byte_buf_writer.md) (`src/lib.rs:99`, via the `write!` macro's `write_fmt` dispatch — see that file's own Caller Tree for the exact mechanism)
- [FmtError](../enum/001_fmt_error.md) (`src/lib.rs:99`, constructed on failure)
- **External:** the `value : impl core::fmt::Display` parameter's own `Display::fmt` (whichever concrete type the caller supplies — `exact_kind::Decimal`/`Qty` in every real usage, driven through the `write!` macro)
