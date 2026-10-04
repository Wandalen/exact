# 003: round_div

## Representation

Divide `n` by `d`, applying `rounding` to a nonzero remainder. A negative `d`
is normalized first (`n / d` with `d < 0` computed as `-n / -d`), so every
sign case only has to handle a positive divisor. Owned here rather than
duplicated into `exact_ratio` and `exact_snap` individually — both already
depend on this crate for `Rounding` itself, and both need the identical
sign-handling and tie-breaking logic (module doc comment, `src/lib.rs:14-22`).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_round/src/lib.rs:118`

```rust
pub const fn round_div( n : i64, d : i64, rounding : Rounding ) -> Result< i64, RoundError >
{
  if d == 0
  {
    return Err( RoundError::DivZero );
  }
  let ( n, d ) = if d < 0
  {
    let Some( neg_n ) = n.checked_neg() else { return Err( RoundError::Overflow ) };
    let Some( neg_d ) = d.checked_neg() else { return Err( RoundError::Overflow ) };
    ( neg_n, neg_d )
  }
  else
  {
    ( n, d )
  };

  let q = n / d;
  let r = n % d;
  if r == 0
  {
    return Ok( q );
  }

  match rounding
  {
    Rounding::Down =>
    {
      if r < 0
      {
        let Some( q ) = q.checked_sub( 1 ) else { return Err( RoundError::Overflow ) };
        Ok( q )
      }
      else
      {
        Ok( q )
      }
    }
    Rounding::Up =>
    {
      if r > 0
      {
        let Some( q ) = q.checked_add( 1 ) else { return Err( RoundError::Overflow ) };
        Ok( q )
      }
      else
      {
        Ok( q )
      }
    }
    Rounding::HalfEven =>
    {
      // `i128::from(_)` is not const-stable on this toolchain — `as` casts are.
      let twice_r_abs = ( r.unsigned_abs() as i128 ) * 2;
      let d_wide = d as i128;
      if twice_r_abs < d_wide
      {
        Ok( q )
      }
      else if twice_r_abs > d_wide || q % 2 != 0
      {
        if n < 0
        {
          let Some( q ) = q.checked_sub( 1 ) else { return Err( RoundError::Overflow ) };
          Ok( q )
        }
        else
        {
          let Some( q ) = q.checked_add( 1 ) else { return Err( RoundError::Overflow ) };
          Ok( q )
        }
      }
      else
      {
        Ok( q )
      }
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 118-196 | Declaration |
| `tests/round_div_test.rs` | throughout | Every rounding mode, both signs, zero-divisor and normalization paths |
| `exact_dust/src/lib.rs:125` | — | **Production** — `split_minor`'s per-share division |
| `exact_snap/src/lib.rs:132,146` | — | **Production** — `price_snap_tick`, `qty_snap_lot` |
| `exact_ratio/src/lib.rs:185` | — | **Production** — `div_round_minor`, backing `money_div_round`/`qty_div_round` |
| `exact_arith/tests/facade_test.rs:56` | — | Re-exported-path test call |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised exhaustively by its own test suite |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the single shared rounding-division primitive behind dust-splitting, price/quantity snapping, and ratio division across all 3 crates |
| `exact_arith` | `tests/facade_test.rs` | Test-only, via the re-exported path |

## Caller Tree

- **External:** `exact_dust::split_minor` (`exact_dust/src/lib.rs:125`)
- **External:** `exact_snap::price_snap_tick` (`exact_snap/src/lib.rs:132`), `qty_snap_lot` (`:146`)
- **External:** `exact_ratio::div_round_minor` (`exact_ratio/src/lib.rs:185`)

No intra-crate caller.

## Callee Tree

- **External:** `i64::checked_neg` (×2 — divisor normalization)
- **External:** `i64::checked_sub`, `i64::checked_add` (×2 each — quotient adjustment in `Down`/`Up`/`HalfEven` branches)
- **External:** `i64::unsigned_abs` — `HalfEven`'s tie detection
