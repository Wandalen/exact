# 002: qty_snap_lot

## Representation

Snap a quantity to the nearest multiple of `lot`, per `rounding`. The
`Quantity` counterpart to [price_snap_tick](001_price_snap_tick.md) — same
round-then-rescale shape.
Under `Rounding::Exact` a quantity not already on the grid is refused as
`SnapError::OffGrid` rather than snapped.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_snap/src/lib.rs:166-172`

```rust
pub fn qty_snap_lot( qty : Quantity, lot : Lot, rounding : Rounding ) -> Result< Quantity, SnapError >
{
  let q = exact_round::round_div( qty.minor(), lot.0.minor(), rounding )
  .map_err( | e | round_error_to_snap_error( e, SnapError::ZeroLot ) )?;
  let snapped = q.checked_mul( lot.0.minor() ).ok_or( SnapError::Overflow )?;
  Quantity::from_minor( snapped ).map_err( | _ | SnapError::Overflow )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 166-172 | Declaration |
| `tests/snap_test.rs` | 120,121,131,132,202-203 | Rounding-down/up parity with `price_snap_tick`, never producing a negative result, and `Exact` keeping an on-grid quantity and refusing an off-grid one |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

No production call site anywhere in the workspace outside `exact_snap`'s own
tests. `exact_arith`'s own facade test suite never calls it either.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `tests/snap_test.rs` | Exercised for rounding parity and the non-negativity guarantee |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Every
call site is test-only, out of scope for Caller/Callee Tree content per
OT012.

## Callee Tree

- `round_error_to_snap_error` (`src/lib.rs:57`, private — no Item Instance of its own)
- **External:** `exact_kind::Qty::minor` (via `qty.minor()` and `lot.0.minor()` ×2)
- **External:** `exact_round::round_div`
- **External:** `i64::checked_mul` (core primitive method, via `q.checked_mul(...)`)
- **External:** `exact_kind::Qty::from_minor` (via `Quantity::from_minor`)
