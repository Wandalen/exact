# 002: RoundError

## Representation

Why a rounded division could not be completed: `DivZero` (a zero divisor),
`Overflow` (the quotient does not fit the integer type — only reachable
dividing the type's minimum value by `-1`), `Inexact` (`Rounding::Exact` was
asked and the division left a remainder).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_round/src/lib.rs:110`

```rust
pub enum RoundError
{
  /// A zero divisor was supplied.
  DivZero,
  /// The quotient does not fit the integer type — only reachable dividing
  /// the type's minimum value by `-1`.
  Overflow,
  /// [`Rounding::Exact`] was asked and the division left a remainder.
  Inexact,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 110,127-129,147,152,171,175,187,212 | Return/constructed variant in `round_div` and `round_div_wide` (`Inexact` from the `Exact` arm), and its `Display` impl |
| `tests/round_div_test.rs` | throughout | `DivZero` refusal, the one `Overflow` (`MIN / -1`), `Inexact` under `Exact` (lines 159-162, 200-220), and the messages |
| `exact_dust/src/lib.rs:113,114,117` | — | **Production** — mapped to `DustError::EmptyParts`/`DustError::Overflow`/`DustError::Remainder` in `round_error_to_dust_error` |
| `exact_snap/src/lib.rs:66-68` | — | **Production** — mapped to a zero-rounding fallback / `SnapError::Overflow` / `SnapError::OffGrid` in `round_error_to_snap_error` |
| `exact_ratio/src/lib.rs:119,123-125` | — | **Production** — mapped to `RatioError::DivZero`/`RatioError::Overflow`/`RatioError::Inexact` in `round_error_to_ratio_error`, shared by `mul_ratio_minor` and `div_round_minor` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | The sole error type of `round_div` and `round_div_wide` |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — each maps `RoundError` into its own local error type via explicit `match`, never a `From` impl |
