# 005: Lot::qty

## Representation

The lot size as a plain `Quantity`. The `Quantity` counterpart to
[Tick::price](003_price_tick.md) — same shape, same fate.

**Never actually called — not even internally.** `qty_snap_lot` (this
crate's one consumer of a `Lot`) reads the wrapped value via `lot.0.minor()`
— direct tuple-field access — rather than `lot.qty().minor()` through this
accessor. Verified by grepping both `src/lib.rs` and `tests/snap_test.rs` for
`.qty()`: zero matches anywhere.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:106-109`

```rust
#[ must_use ]
pub const fn qty( self ) -> Quantity
{
  self.0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 106-109 | Declaration |
| `exact_arith/src/lib.rs:123` | — | Facade re-export (via `Lot`'s re-export) |

No call site anywhere, production or test — an honest empty finding, matching
[Tick::price](003_price_tick.md) exactly: declared, part of the public API,
dead even from its own crate's internal logic.

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
