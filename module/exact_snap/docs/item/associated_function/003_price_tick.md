# 003: Tick::price

## Representation

The tick size as a plain `Price`. Named identically in shape to
[Lot::qty](005_qty_lot.md) — both are the one-field tuple-struct accessor for
their respective grid-spacing type; filenames carry the `_tick`/`_lot` suffix
for consistency with [Tick::new](002_new_tick.md)/[Lot::new](004_new_lot.md),
even though `price`/`qty` are distinct names that don't themselves collide.

**Never actually called — not even internally.** `price_snap_tick` (this
crate's one consumer of a `Tick`) reads the wrapped value via `tick.0.minor()`
— direct tuple-field access, legal because `price_snap_tick` shares `Tick`'s
defining module — rather than `tick.price().minor()` through this accessor.
Verified by grepping both `src/lib.rs` and `tests/snap_test.rs` for
`.price()`: zero matches anywhere.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:78-81`

```rust
#[ must_use ]
pub const fn price( self ) -> Price
{
  self.0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 78-81 | Declaration |
| `exact_arith/src/lib.rs:117` | — | Facade re-export (via `Tick`'s re-export; the method itself is not separately named in the `pub use`) |

No call site anywhere, production or test — an honest empty finding, and the
sharpest one in this crate: the accessor exists and is part of the public
API, but is dead even from its own crate's internal logic.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | Declared, never invoked |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission. See the discrepancy note above.

## Callee Tree

No callees — the method body is a single field projection (`self.0`), no
function call.
