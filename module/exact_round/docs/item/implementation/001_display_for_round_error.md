# 001: impl Display for RoundError

## Representation

Renders each `RoundError` variant as a specific sentence: "a zero divisor was
supplied" / "the quotient does not fit the integer type".

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_round/src/lib.rs:121`

```rust
impl core::fmt::Display for RoundError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::DivZero => write!( f, "a zero divisor was supplied" ),
      Self::Overflow => write!( f, "the quotient does not fit the integer type" ),
      Self::Inexact => write!( f, "the division left a remainder and Rounding::Exact was requested" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 121-132 | Declaration |

Rendered only by this crate's own tests (`tests/round_div_test.rs:248,255`,
via `.to_string()`), which pin both messages. Every downstream crate maps
`RoundError` into its own local error type by `match` reconstruction, never
by rendering the message (see [RoundError](../enum/002_round_error.md)'s
Crate Usage).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Declared here; both messages pinned by its own tests |
