# 001: price_snap_tick

## Representation

Snap a price to the nearest multiple of `tick`, per `rounding`. Divides to
find the nearest grid index (sign-normalized, tie-broken by `rounding`), then
multiplies back out — the same round-then-rescale shape `exact_ratio`'s
`*_div_round` functions use, built on the same shared `exact_round::round_div`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_snap/src/lib.rs:118-124`

```rust
pub fn price_snap_tick( price : Price, tick : Tick, rounding : Rounding ) -> Result< Price, SnapError >
{
  let q = exact_round::round_div( price.minor(), tick.0.minor(), rounding )
  .map_err( | e | round_error_to_snap_error( e, SnapError::ZeroTick ) )?;
  let snapped = q.checked_mul( tick.0.minor() ).ok_or( SnapError::Overflow )?;
  Price::from_minor( snapped ).map_err( | _ | SnapError::Overflow )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 118-124 | Declaration |
| `tests/snap_test.rs` | 25,35,44,53 | On-grid identity, between-grid rounding down/up, and half-even tie-breaking |
| `exact_arith/src/lib.rs:117` | — | Facade re-export |

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

- `round_error_to_snap_error` (`src/lib.rs:42`, private — no Item Instance of its own)
- **External:** `exact_kind::Decimal::minor` (`Price` aliases `Decimal`, via `price.minor()` and `tick.0.minor()` ×2)
- **External:** `exact_round::round_div`
- **External:** `i64::checked_mul` (core primitive method, via `q.checked_mul(...)`)
- **External:** `exact_kind::Decimal::from_minor` (via `Price::from_minor`)
