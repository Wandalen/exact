# 001: impl Display for KindError

## Representation

Renders each `KindError` variant as a specific, investigable sentence rather
than a generic failure — e.g. `"{minor} minor units exceeds the declared
ceiling {CEILING_MINOR_UNITS}"`, naming the actual offending value and the
actual limit.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:123`

```rust
impl fmt::Display for KindError
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Self::Overflow { operation } => write!( f, "{operation} left the representable range" ),
      Self::ExceedsCeiling { minor } => write!( f, "{minor} minor units exceeds the declared ceiling {CEILING_MINOR_UNITS}" ),
      Self::ExcessPrecision { digits, scale } => write!( f, "{digits} fractional digits into a type of scale {scale}" ),
      Self::Malformed { reason } => write!( f, "malformed decimal: {reason}" ),
      Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 123-136 | Declaration |

No file anywhere in the workspace calls `KindError`'s `Display::fmt`
explicitly or via `.to_string()`/format interpolation — the method named in
the `## Associated Function/Method` entry for `fmt` (see
[fmt_display_for_kind_error](../associated_function/020_fmt_display_for_kind_error.md))
carries the honest-empty finding.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Error rendering for every fallible operation's failure path |
