# 006: price_sub

## Representation

Subtract two prices, dispatching straight to `exact_kind`'s own checked
subtraction — `Price::checked_sub`, which delegates to the same operation as
[`money_sub`](002_money_sub.md) on the `Money` it wraps.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:103`

```rust
pub const fn price_sub( a : Price, b : Price ) -> Result< Price, KindError >
{
  a.checked_sub( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 103 | Declaration |
| `tests/checked_and_saturating_add_test.rs:43` | — | Dispatch parity with `price_add`, confirming a price subtracts exactly as money does |
| `exact_arith/src/lib.rs:96` | — | Facade re-export |

No production (non-test) file outside `exact_add` calls `price_sub` — an
honest empty finding.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own dispatch-parity test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own test — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Price::checked_sub` (via `a.checked_sub( b )`), which delegates to `exact_kind::Decimal::checked_sub`
