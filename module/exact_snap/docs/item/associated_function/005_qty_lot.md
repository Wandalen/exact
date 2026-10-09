# 005: Lot::qty

## Representation

The lot size as a plain `Quantity`. The `Quantity` counterpart to
[Tick::price](003_price_tick.md) — same shape, same fate.

**Never called in production — only by a test.** `qty_snap_lot` (this
crate's one consumer of a `Lot`) reads the wrapped value via `lot.0.minor()`
— direct tuple-field access — rather than `lot.qty().minor()` through this
accessor. Verified by grepping `src/lib.rs` and `tests/snap_test.rs` for
`.qty()`: the only match is `tests/snap_test.rs:161`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:128-131`

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
| `src/lib.rs` | 128-131 | Declaration |
| `tests/snap_test.rs` | 161 | A lot hands back the size it was built from |
| `exact_arith/src/lib.rs:135` | — | Facade re-export (via `Lot`'s re-export) |

No production call site anywhere — an honest empty finding, matching
[Tick::price](003_price_tick.md) exactly: declared, part of the public API,
dead even from its own crate's internal logic; only a test calls it.

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
