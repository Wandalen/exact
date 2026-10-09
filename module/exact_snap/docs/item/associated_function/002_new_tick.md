# 002: Tick::new

## Representation

Build a tick size, refusing a zero-sized one and storing a negative one as
its magnitude, so a tick of -5 is the same tick as a tick of 5. Named identically to
[Lot::new](004_new_lot.md) — a same-crate identifier collision, disambiguated
by the `_tick`/`_lot` filename suffix per the established `exact_kind`
convention (`Decimal`/`Qty`'s shared method names, e.g.
`exact_kind/docs/item/associated_function/001_from_minor_decimal.md` /
`011_from_minor_qty.md`).

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:84-96`

```rust
pub const fn new( price : Price ) -> Result< Self, SnapError >
{
  if price.minor() == 0
  {
    return Err( SnapError::ZeroTick );
  }
  // The price range is symmetric about zero, so the magnitude always fits.
  match Price::from_minor( price.minor().abs() )
  {
    Ok( size ) => Ok( Self( size ) ),
    Err( _ ) => Err( SnapError::Overflow ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 84-96 | Declaration |
| `tests/snap_test.rs` | 12,21,33,42,51,100,157-159 | Zero-rejection check; the fixture every snap test builds before calling `price_snap_tick`; a tick of -5 stored as 5 and equal to a tick of 5 |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

No production call site anywhere in the workspace outside `exact_snap`'s own
tests — `exact_arith`'s own facade test suite never constructs a `Tick`
either. An honest empty finding for production usage; real usage is confined
to this crate's own test fixtures.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `tests/snap_test.rs` | The only way to build a `Tick`; every test fixture goes through it |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Every
call site is test-only (`tests/snap_test.rs`), which is out of scope for
Caller/Callee Tree content per OT012 (production call-graph structure only).

## Callee Tree

- **External:** `exact_kind::Price::minor` (via `price.minor()`)
- **External:** `i64::abs` (core primitive method, via `price.minor().abs()`)
- **External:** `exact_kind::Price::from_minor`
