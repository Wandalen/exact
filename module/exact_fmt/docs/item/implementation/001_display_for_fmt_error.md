# 001: Display for FmtError

## Representation

Renders `FmtError`'s one variant as a human-readable sentence.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_fmt/src/lib.rs:37-46`

```rust
impl core::fmt::Display for FmtError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::BufFull => write!( f, "the buffer was too small to hold the rendered text" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 37-46 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | The one hand-written trait impl on `FmtError`; see [Display::fmt for FmtError](../associated_function/001_fmt_display_for_fmt_error.md) for its method-level usage evidence (never rendered anywhere, including this crate's own tests) |
