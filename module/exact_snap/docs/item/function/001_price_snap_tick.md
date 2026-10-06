# 001: price_snap_tick

## Representation

Snap a price to the nearest multiple of `tick`, per `rounding`. Divides to
find the nearest grid index (sign-handled, tie-broken by `rounding`), then
multiplies back out — the same round-then-rescale shape `exact_ratio`'s
`*_div_round` functions use, built on the same shared `exact_round::round_div`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_snap/src/lib.rs:130-146`

```rust
pub fn price_snap_tick( price : Price, tick : Tick, rounding : Rounding ) -> Result< Price, SnapError >
{
  // Fix(exact_snap_negative_tick_reversed_rounding): the price used to be
  // divided by the signed tick, so for a tick of -5 the count was rounded on
  // a reversed axis and multiplied back — `Down` snapped up and `Up` down.
  // A tick of -5 marks the same grid as a tick of 5; dividing by the
  // positive spacing keeps `Down` meaning the grid point at or below.
  //
  // Root cause: rounding a quotient by a negative divisor flips its direction.
  // Pitfall: `Down`/`Up` round the quotient toward -∞/+∞; multiplied back by
  //   a negative spacing, that direction reverses for the value itself.
  let spacing = tick.0.minor().abs();
  let q = exact_round::round_div( price.minor(), spacing, rounding )
  .map_err( | e | round_error_to_snap_error( e, SnapError::ZeroTick ) )?;
  let snapped = q.checked_mul( spacing ).ok_or( SnapError::Overflow )?;
  Price::from_minor( snapped ).map_err( | _ | SnapError::Overflow )
}
```

The division and the multiplication use the tick's magnitude, so a negative
tick — which `Tick::new` accepts — snaps exactly like its positive
counterpart instead of reversing `Down` and `Up`.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 130-146 | Declaration |
| `tests/snap_test.rs` | 25,35,44,53,63-64,74-75,106,146 | On-grid identity, between-grid rounding down/up, half-even on and off a tie, a negative price, a negative tick matching its positive counterpart, and overflow past the ceiling |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

No production call site anywhere in the workspace outside `exact_snap`'s own
tests — confirmed via grep across `substrate/` and `module/`.
`exact_arith`'s own facade test suite never calls it either, the same
facade-bypass pattern already found in `exact_add`/`exact_parse`'s catalogs.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `tests/snap_test.rs` | Exercised across all 3 rounding modes and the on-grid identity case |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Every
call site is test-only, out of scope for Caller/Callee Tree content per
OT012.

## Callee Tree

- `round_error_to_snap_error` (`src/lib.rs:54`, private — no Item Instance of its own)
- **External:** `exact_kind::Price::minor` (via `price.minor()` and `tick.0.minor()`), which delegates to `Decimal::minor`
- **External:** `i64::abs` (core primitive method, via `tick.0.minor().abs()`)
- **External:** `exact_round::round_div`
- **External:** `i64::checked_mul` (core primitive method, via `q.checked_mul(...)`)
- **External:** `exact_kind::Decimal::from_minor` (via `Price::from_minor`)
