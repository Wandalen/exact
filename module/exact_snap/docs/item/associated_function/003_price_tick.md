# 003: Tick::price

## Representation

The tick size as a plain `Price` — always positive, since `Tick::new` stores
a negative tick as its magnitude. Named identically in shape to
[Lot::qty](005_qty_lot.md) — both are the one-field tuple-struct accessor for
their respective grid-spacing type; filenames carry the `_tick`/`_lot` suffix
for consistency with [Tick::new](002_new_tick.md)/[Lot::new](004_new_lot.md),
even though `price`/`qty` are distinct names that don't themselves collide.

**Never called in production — only by a test.** `price_snap_tick` (this
crate's one consumer of a `Tick`) reads the wrapped value via `tick.0.minor()`
— direct tuple-field access, legal because `price_snap_tick` shares `Tick`'s
defining module — rather than `tick.price().minor()` through this accessor.
Verified by grepping `src/lib.rs` and `tests/snap_test.rs` for `.price()`: the
only matches are `tests/snap_test.rs:157-158`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:100-103`

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
| `src/lib.rs` | 100-103 | Declaration |
| `tests/snap_test.rs` | 157-158 | A tick built from 5 and one built from -5 both report a size of 5 |
| `exact_arith/src/lib.rs:135` | — | Facade re-export (via `Tick`'s re-export; the method itself is not separately named in the `pub use`) |

No production call site anywhere — an honest empty finding, and the
sharpest one in this crate: the accessor exists and is part of the public
API, but is dead even from its own crate's internal logic; only a test calls it.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `tests/snap_test.rs` | Declared here; invoked only by the size test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission. See the discrepancy note above.

## Callee Tree

No callees — the method body is a single field projection (`self.0`), no
function call.
