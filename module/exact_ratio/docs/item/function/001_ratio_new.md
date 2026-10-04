# 001: ratio_new

## Representation

Build a `Ratio`, refusing a zero denominator. A negative denominator is
accepted and normalized: `n / d` with `d < 0` is stored as `-n / -d`, so
every later rounding calculation can assume a positive denominator.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:112`

```rust
pub const fn ratio_new( n : i64, d : i64 ) -> Result< Ratio, RatioError >
{
  if d == 0
  {
    return Err( RatioError::DivZero );
  }
  if d < 0
  {
    let Some( neg_n ) = n.checked_neg() else { return Err( RatioError::Overflow ) };
    let Some( neg_d ) = d.checked_neg() else { return Err( RatioError::Overflow ) };
    return Ok( Ratio { n : neg_n, d : neg_d } );
  }
  Ok( Ratio { n, d } )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 112-125 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 12,19,29,39,49 | Zero-denominator refusal, negative-denominator normalization, and the sole construction path for every other test's `Ratio` value |
| `exact_arith/src/lib.rs:102` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | The only public constructor for `Ratio` |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Every
call site is in `exact_ratio`'s own tests, which are out of scope for this
tree (production call-graph only); `exact_arith` only re-exports the name.

## Callee Tree

- **External:** `i64::checked_neg` (×2 — negative-denominator normalization)

No Defining-Crate function calls — the `Ok`/`Err` arms construct `Ratio`
directly as a struct literal rather than delegating.
