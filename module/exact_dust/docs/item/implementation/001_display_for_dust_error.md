# 001: Display for DustError

## Representation

Renders each `DustError` variant as a human-readable sentence.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_dust/src/lib.rs:75-86`

```rust
impl core::fmt::Display for DustError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::EmptyParts => write!( f, "cannot split into zero parts" ),
      Self::Remainder => write!( f, "the split left a remainder and DustTo::Reject was requested" ),
      Self::Overflow => write!( f, "left the representable or declared range" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 75-86 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | The one hand-written trait impl on `DustError`; see [Display::fmt for DustError](../associated_function/001_fmt_display_for_dust_error.md) for its method-level usage evidence |
