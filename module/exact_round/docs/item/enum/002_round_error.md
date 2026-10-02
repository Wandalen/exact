# 002: RoundError

## Representation

Why a rounded division could not be completed: `DivZero` (a zero divisor),
`Overflow` (normalizing a negative divisor or adjusting the quotient by one
overflowed — only reachable at `i64::MIN`/`i64::MAX`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_round/src/lib.rs:75`

```rust
pub enum RoundError
{
  DivZero,
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 75,90,91,109,113,114,117,118,139,151,172,177 | Return/constructed variant throughout `round_div` and its `Display` impl |
| `tests/round_div_test.rs` | throughout | `DivZero` refusal check |
| `exact_dust/src/lib.rs:99,100` | — | **Production** — mapped to `DustError::EmptyParts`/`DustError::Overflow` in `round_error_to_dust_error` |
| `exact_snap/src/lib.rs:51,52` | — | **Production** — mapped to a zero-rounding fallback / `SnapError::Overflow` |
| `exact_ratio/src/lib.rs:175,176` | — | **Production** — mapped to `RatioError::DivZero`/`RatioError::Overflow` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | `round_div`'s sole error type |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — each maps `RoundError` into its own local error type via explicit `match`, never a `From` impl |
