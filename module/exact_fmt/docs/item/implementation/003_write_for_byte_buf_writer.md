# 003: Write for ByteBufWriter

## Representation

Makes `ByteBufWriter` usable as the target of the standard `write!` macro —
the one piece of plumbing that lets `fmt_into` drive an existing `Display`
impl into a byte slice with no allocation.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_fmt/src/lib.rs:70-83`

```rust
impl core::fmt::Write for ByteBufWriter< '_ >
{
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
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 70-83 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | The one hand-written trait impl on `ByteBufWriter`; see [write_str for ByteBufWriter](../associated_function/002_write_str_for_byte_buf_writer.md) for its method-level usage evidence |
