# 002: RoundError

## Representation

Why a rounded division could not be completed: `DivZero` (a zero divisor),
`Overflow` (the quotient does not fit the integer type — only reachable
dividing the type's minimum value by `-1`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_round/src/lib.rs:104`

```rust
pub enum RoundError
{
  /// A zero divisor was supplied.
  DivZero,
  /// The quotient does not fit the integer type — only reachable dividing
  /// the type's minimum value by `-1`.
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 104,119,120,136,141,158,162,174 | Return/constructed variant in `round_div` and `round_div_wide`, and its `Display` impl |
| `tests/round_div_test.rs` | throughout | `DivZero` refusal, the one `Overflow` (`MIN / -1`), and both messages |
| `exact_dust/src/lib.rs:112,113` | — | **Production** — mapped to `DustError::EmptyParts`/`DustError::Overflow` in `round_error_to_dust_error` |
| `exact_snap/src/lib.rs:63,64` | — | **Production** — mapped to a zero-rounding fallback / `SnapError::Overflow` |
| `exact_ratio/src/lib.rs:196,197` | — | **Production** — mapped to `RatioError::DivZero`/`RatioError::Overflow` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | The sole error type of `round_div` and `round_div_wide` |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — each maps `RoundError` into its own local error type via explicit `match`, never a `From` impl |
