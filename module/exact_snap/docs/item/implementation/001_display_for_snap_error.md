# 001: Display for SnapError

## Representation

Renders each of `SnapError`'s 3 variants as a human-readable sentence.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_snap/src/lib.rs:39-50`

```rust
impl core::fmt::Display for SnapError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::ZeroTick => write!( f, "a zero-sized tick was supplied" ),
      Self::ZeroLot => write!( f, "a zero-sized lot was supplied" ),
      Self::Overflow => write!( f, "the snapped result left the representable or declared range" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 39-50 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | The one hand-written trait impl on `SnapError`; see [Display::fmt for SnapError](../associated_function/001_fmt_display_for_snap_error.md) for method-level usage evidence (never rendered anywhere, including this crate's own tests) |
