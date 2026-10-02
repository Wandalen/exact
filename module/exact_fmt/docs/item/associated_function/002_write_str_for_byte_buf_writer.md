# 002: write_str for ByteBufWriter

## Representation

Appends `s`'s bytes to the buffer if there's room, refusing (via
`core::fmt::Error`) rather than truncating when there isn't.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_fmt/src/lib.rs:58-68`

```rust
fn write_str( &mut self, s : &str ) -> core::fmt::Result
{
  let bytes = s.as_bytes();
  if self.len + bytes.len() > self.buf.len()
  {
    return Err( core::fmt::Error );
  }
  self.buf[ self.len .. self.len + bytes.len() ].copy_from_slice( bytes );
  self.len += bytes.len();
  Ok( () )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 58-68 | Declaration |
| `tests/fmt_test.rs:34` | — | Indirectly exercises the `Err` branch (too-small buffer) through `fmt_into` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | The one write sink `fmt_into` renders through, exercised (both branches) by this crate's own tests |

## Caller Tree

- [fmt_into](../function/001_fmt_into.md) (`src/lib.rs:85`) — **not a direct
  call in the source text**: `write!( writer, "{value}" )` expands to
  `writer.write_fmt( format_args!( "{value}" ) )`, whose default
  implementation (`core::fmt::Write`, external) drives `value`'s own
  `Display::fmt` against a `Formatter` wrapping `writer`, which in turn calls
  back into this method. A dispatch-mechanism hop through the standard
  library's formatting machinery, not a literal `writer.write_str( ... )`
  call anywhere in this crate's source.

## Callee Tree

No callee of its own — reads `self.buf`/`self.len` and `core::primitive`
slice operations (`copy_from_slice`) directly; no further hop into another
Item Instance.
