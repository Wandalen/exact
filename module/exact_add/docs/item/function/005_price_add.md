# 005: price_add

## Representation

Add two prices, dispatching straight to `exact_kind`'s own checked addition
— `Price::checked_add`, which delegates to the same operation as
[`money_add`](001_money_add.md) on the `Money` it wraps.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:93`

```rust
pub const fn price_add< const S : u32 >( a : PriceAt< S >, b : PriceAt< S > ) -> Result< PriceAt< S >, KindError >
{
  a.checked_add( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 93 | Declaration |
| `tests/checked_and_saturating_add_test.rs:42` | — | Dispatch parity with `price_sub`, confirming a price adds exactly as money does |
| `exact_arith/src/lib.rs:96` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:20` | — | Adds two prices in cents |

No production (non-test) file outside `exact_add` calls `price_add` — an
honest empty finding.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own dispatch-parity test |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export; called at scale 2 by the any-scale facade test |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own test — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Price::checked_add` (via `a.checked_add( b )`), which delegates to `exact_kind::Decimal::checked_add`
