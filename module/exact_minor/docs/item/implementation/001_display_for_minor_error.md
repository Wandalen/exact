# 001: impl Display for MinorError

## Representation

Renders the one variant as `"{operation} left the representable range"`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_minor/src/lib.rs:204`

```rust
impl fmt::Display for MinorError
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Self::Overflow { operation } => write!( f, "{operation} rose above the representable range" ),
      Self::Underflow { operation } => write!( f, "{operation} fell below the representable range" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 204-214 | Declaration |
| `tests/checked_arithmetic_test.rs` | `overflow_error_names_the_failed_operation` | Exact rendered text of a `neg` overflow |

No production code anywhere in the workspace renders a `MinorError` via
`.to_string()` or format interpolation — confirmed by grep. This crate's own
`tests/checked_arithmetic_test.rs` renders one and checks the exact text. See
[Display::fmt for MinorError](../associated_function/001_fmt_display_for_minor_error.md)
for the method's own honest-empty Caller Tree.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Error rendering for `MinorError` |
