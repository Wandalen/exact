# 001: Display for WireError

## Representation

Renders each of `WireError`'s 5 variants as a human-readable sentence.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_bytes/src/lib.rs:75-88`

```rust
impl core::fmt::Display for WireError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::BadKind => write!( f, "the wire's kind byte did not match the kind being decoded into" ),
      Self::BadScale => write!( f, "the wire's scale byte did not match the kind's expected scale" ),
      Self::Truncated => write!( f, "the byte slice was shorter than the wire encoding's fixed length" ),
      Self::Overflow => write!( f, "the decoded value left the representable or declared range" ),
      Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 75-88 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | The one hand-written trait impl on `WireError`; see [Display::fmt for WireError](../associated_function/001_fmt_display_for_wire_error.md) for method-level usage evidence (never rendered anywhere) |
