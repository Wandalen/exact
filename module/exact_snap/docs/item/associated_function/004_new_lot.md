# 004: Lot::new

## Representation

Build a lot size, refusing a zero-sized one. The `Quantity` counterpart to
[Tick::new](002_new_tick.md) — same same-crate name collision, same
`_tick`/`_lot` disambiguation.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:113-120`

```rust
pub const fn new( qty : Quantity ) -> Result< Self, SnapError >
{
  if qty.minor() == 0
  {
    return Err( SnapError::ZeroLot );
  }
  Ok( Self( qty ) )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 113-120 | Declaration |
| `tests/snap_test.rs` | 13,117,128 | Zero-rejection check, and as the fixture every `qty_snap_lot` test builds |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

No production call site anywhere in the workspace outside `exact_snap`'s own
tests — `exact_arith`'s own facade test suite never constructs a `Lot`
either. An honest empty finding for production usage.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `tests/snap_test.rs` | The only way to build a `Lot`; every test fixture goes through it |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Every
call site is test-only, out of scope for Caller/Callee Tree content per
OT012.

## Callee Tree

- **External:** `exact_kind::Quantity::minor` (via `qty.minor()`)
