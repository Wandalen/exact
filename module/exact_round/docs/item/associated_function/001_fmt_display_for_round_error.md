# 001: Display::fmt for RoundError

## Representation

Renders each `RoundError` variant's message. See
[impl Display for RoundError](../implementation/001_display_for_round_error.md)
for the full body.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_round/src/lib.rs:115`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::DivZero => write!( f, "a zero divisor was supplied" ),
    Self::Overflow => write!( f, "the quotient does not fit the integer type" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 115-122 | Declaration |
| `tests/round_div_test.rs` | 248,255 | Both messages, via `.to_string()` |

No production file calls this method — every downstream crate maps
`RoundError` by `match`, never by rendering it (see
[RoundError](../enum/002_round_error.md)'s Crate Usage).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Declared here; invoked only by its own tests |

## Caller Tree

- **Test-only:** `the_overflow_message_names_the_real_cause`, `the_div_zero_message_names_the_zero_divisor` (`tests/round_div_test.rs:248,255`)

## Callee Tree

- **External:** `fmt::Formatter::write_fmt` (×2, one per variant, via the
  `write!` macro)
