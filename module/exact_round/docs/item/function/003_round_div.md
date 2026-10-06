# 003: round_div

## Representation

Divide `n` by `d`, applying `rounding` to a nonzero remainder. The `i64` form
of [round_div_wide](004_round_div_wide.md): every `i64` fits an `i128`, so it
widens both operands, lets `round_div_wide` apply the rounding rules — written
once, there — and narrows the result back, reporting `Overflow` only for the
one quotient no `i64` holds, `i64::MIN / -1`. Owned here rather than
duplicated into `exact_ratio`, `exact_snap` and `exact_dust` individually —
they already depend on this crate for `Rounding` itself, and they need the
identical sign-handling and tie-breaking logic (module doc comment,
`src/lib.rs:14-24`).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_round/src/lib.rs:118`

```rust
pub const fn round_div( n : i64, d : i64, rounding : Rounding ) -> Result< i64, RoundError >
{
  match round_div_wide( n as i128, d as i128, rounding )
  {
    Ok( q ) if q >= i64::MIN as i128 && q <= i64::MAX as i128 => Ok( q as i64 ),
    Ok( _ ) => Err( RoundError::Overflow ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 118-126 | Declaration |
| `tests/round_div_test.rs` | throughout | Every rounding mode, both signs, the zero divisor, the minimum value as either operand, and a grid checked against each mode's definition |
| `exact_dust/src/lib.rs:125` | — | **Production** — `split_minor`'s per-share division |
| `exact_snap/src/lib.rs:143,156` | — | **Production** — `price_snap_tick`, `qty_snap_lot` |
| `exact_ratio/src/lib.rs:193` | — | **Production** — `div_round_minor`, backing `money_div_round`/`qty_div_round` |
| `exact_arith/tests/facade_test.rs:56` | — | Re-exported-path test call |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised exhaustively by its own test suite |
| `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the shared `i64` rounding division behind dust-splitting, price/quantity snapping, and ratio division across all 3 crates |
| `exact_arith` | `tests/facade_test.rs` | Test-only, via the re-exported path |

## Caller Tree

- **External:** `exact_dust::split_minor` (`exact_dust/src/lib.rs:125`)
- **External:** `exact_snap::price_snap_tick` (`exact_snap/src/lib.rs:143`), `qty_snap_lot` (`:156`)
- **External:** `exact_ratio::div_round_minor` (`exact_ratio/src/lib.rs:193`)

No intra-crate caller.

## Callee Tree

- [round_div_wide](004_round_div_wide.md) (`src/lib.rs:120`) — the rounding itself, over the widened operands
