# 004: round_div_wide

## Representation

Divide `n` by `d` over `i128`, applying `rounding` to a nonzero remainder —
the one implementation of the rounding rules. [round_div](003_round_div.md)
calls it with widened operands, and `exact_ratio` calls it directly for a
product of two `i64` values, which no `i64` can hold. The operands keep their
signs: a truncating division gives `q`, and the rounding direction comes from
the signs of the remainder and the divisor, so a minimum-value operand rounds
like any other. `Overflow` is reported only for `i128::MIN / -1`, the one
quotient no `i128` holds; the final one-step adjustment cannot overflow,
because a nonzero remainder needs `|d| >= 2`. The tie detection the three
`Half*` modes share doubles the remainder in `u128`, because twice a remainder just below
`i128::MAX` would not fit `i128`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_round/src/lib.rs:158`

```rust
pub const fn round_div_wide( n : i128, d : i128, rounding : Rounding ) -> Result< i128, RoundError >
{
  if d == 0
  {
    return Err( RoundError::DivZero );
  }
  // Fix(exact_round_minimum_value_refused): a negative divisor used to be
  // handled by negating both operands first, which has no result for the
  // type's `MIN` (`i64::MIN` in `round_div`, `i128::MIN` here), so `(0, MIN)`,
  // `(1, MIN)`, `(MIN, -2)` and `(MIN, MIN)` returned `Overflow` though each
  // quotient fits. The operands now keep their signs; only the quotient can overflow.
  //
  // Root cause: `checked_neg` on an operand, where only the quotient can overflow.
  // Pitfall: two's-complement `MIN` has no positive counterpart — normalising
  //   signs by negation fails exactly at the edge a test grid rarely reaches.
  // Truncates toward zero; `MIN / -1` is the one quotient with no representable result.
  let Some( q ) = n.checked_div( d ) else { return Err( RoundError::Overflow ) };
  let r = n % d;
  if r == 0
  {
    return Ok( q );
  }

  // The exact quotient lies strictly between `q` and its neighbour one step
  // further from zero: below `q` when the remainder and divisor differ in sign.
  let exact_is_below = ( r < 0 ) != ( d < 0 );
  // How the remainder compares with half the divisor, without fractions.
  // `u128`: twice a remainder just below `i128::MAX` would not fit `i128`.
  let twice_r = r.unsigned_abs() * 2;
  let d_abs = d.unsigned_abs();
  // `q` was truncated toward zero, so stepping toward the exact quotient
  // always moves one step away from zero.
  let step_toward_exact = match rounding
  {
    Rounding::Down => exact_is_below,
    Rounding::Up => !exact_is_below,
    Rounding::TowardZero => false,
    Rounding::AwayFromZero => true,
    Rounding::HalfEven => twice_r > d_abs || ( twice_r == d_abs && q % 2 != 0 ),
    Rounding::HalfUp => twice_r >= d_abs,
    Rounding::HalfDown => twice_r > d_abs,
  };
  if !step_toward_exact
  {
    return Ok( q );
  }
  // Cannot overflow: a nonzero remainder needs `|d| >= 2`, so `|q| <= |n| / 2`.
  Ok( if exact_is_below { q - 1 } else { q + 1 } )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 158-206 | Declaration |
| `src/lib.rs` | 138 | `round_div`'s call, with widened operands |
| `tests/round_div_test.rs` | 4,155,196-203,234-240 | Each mode's definition on a grid; a dividend wider than `i64`, with positive and negative divisors; the minimum value as either operand; the zero divisor |
| `exact_ratio/src/lib.rs:150-151` | — | **Production** — `mul_ratio_minor`, backing `money_mul_ratio`/`qty_mul_ratio`/`price_mul_ratio`/`price_mul_qty` |
| `exact_arith/src/lib.rs:84` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Called by `round_div`; exercised by its own test suite |
| `exact_ratio` | `src/lib.rs` | **Production** — rounds every ratio multiply per the caller's mode |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- [round_div](003_round_div.md) (`src/lib.rs:138`)
- **External:** `exact_ratio::mul_ratio_minor` (`exact_ratio/src/lib.rs:150-151`)

## Callee Tree

- **External:** `i128::checked_div` — the truncating division, failing only for `i128::MIN / -1`
- **External:** `i128::unsigned_abs` (×2) — `HalfEven`'s tie detection
