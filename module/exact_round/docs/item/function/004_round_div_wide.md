# 004: round_div_wide

## Representation

[round_div](003_round_div.md) over `i128`, for a dividend no `i64` can hold.
`exact_ratio` multiplies two `i64` values in `i128` before dividing, so the
dividend it divides does not fit `round_div`'s `i64`. The rounding rules are
the same; `tests/round_div_test.rs`'s `round_div_wide_agrees_with_round_div`
checks the two agree on every input both accept, so the two copies cannot
drift apart unnoticed.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_round/src/lib.rs:209`

```rust
pub const fn round_div_wide( n : i128, d : i128, rounding : Rounding ) -> Result< i128, RoundError >
```

The body is `round_div`'s, line for line, over `i128`, with one difference:
`HalfEven`'s tie detection doubles the remainder in `u128`, because twice a
remainder just below `i128::MAX` would not fit `i128`.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 209-287 | Declaration |
| `tests/round_div_test.rs` | 4,100,111-113 | Agreement with `round_div`; a dividend wider than `i64`; the zero divisor |
| `exact_ratio/src/lib.rs:143` | — | **Production** — `mul_ratio_minor`, backing `money_mul_ratio`/`qty_mul_ratio`/`price_mul_ratio` |
| `exact_arith/src/lib.rs:77` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised by its own test suite |
| `exact_ratio` | `src/lib.rs` | **Production** — rounds every ratio multiply per the caller's mode |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_ratio::mul_ratio_minor` (`exact_ratio/src/lib.rs:143`)

No intra-crate caller.

## Callee Tree

- **External:** `i128::checked_neg` (×2 — divisor normalization)
- **External:** `i128::checked_sub`, `i128::checked_add` (×2 each — quotient adjustment in `Down`/`Up`/`HalfEven` branches)
- **External:** `i128::unsigned_abs` (×2) — `HalfEven`'s tie detection
