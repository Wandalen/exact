# 001: impl Display for RatioError

## Representation

Renders a human-readable message for each `RatioError` variant. Carries one
associated function (`fmt`) — see its own catalog entry under
`associated_function/`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_ratio/src/lib.rs:65`

```rust
impl core::fmt::Display for RatioError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::DivZero => write!( f, "a zero denominator was supplied" ),
      Self::Overflow => write!( f, "left the representable or declared range" ),
      Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 65-76 | Declaration — the block spans the one `fmt` member |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Declares `RatioError`'s `Display` surface |
