# 002: RoundError

## Representation

Why a rounded division could not be completed: `DivZero` (a zero divisor),
`Overflow` (normalizing a negative divisor overflowed — only reachable when
negating the type's minimum value).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_round/src/lib.rs:84`

```rust
pub enum RoundError
{
  /// A zero divisor was supplied.
  DivZero,
  /// Normalizing a negative divisor overflowed: negating the type's minimum
  /// value, the only overflow a rounded division can reach.
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 84,99,100,118,122,123,126,127,148,160,181,186 | Return/constructed variant throughout `round_div` and its `Display` impl |
| `tests/round_div_test.rs` | throughout | `DivZero` refusal check |
| `exact_dust/src/lib.rs:112,113` | — | **Production** — mapped to `DustError::EmptyParts`/`DustError::Overflow` in `round_error_to_dust_error` |
| `exact_snap/src/lib.rs:63,64` | — | **Production** — mapped to a zero-rounding fallback / `SnapError::Overflow` |
| `exact_ratio/src/lib.rs:188,189` | — | **Production** — mapped to `RatioError::DivZero`/`RatioError::Overflow` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | `round_div`'s sole error type |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — each maps `RoundError` into its own local error type via explicit `match`, never a `From` impl |
