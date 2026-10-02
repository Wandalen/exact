# 001: impl Display for RoundError

## Representation

Renders each `RoundError` variant as a specific sentence: "a zero divisor was
supplied" / "adjusting the quotient for the chosen rounding mode overflowed".

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_round/src/lib.rs:84`

```rust
impl core::fmt::Display for RoundError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::DivZero => write!( f, "a zero divisor was supplied" ),
      Self::Overflow => write!( f, "adjusting the quotient for the chosen rounding mode overflowed" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 84-94 | Declaration |

No file anywhere in the workspace calls this method explicitly or via
`.to_string()`/format interpolation — confirmed via grep. Every downstream
crate maps `RoundError` into its own local error type by `match`
reconstruction, never by rendering the message (see
[RoundError](../enum/002_round_error.md)'s Crate Usage).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Declared here; not exercised by its own test suite either |
