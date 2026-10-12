# 001: Money

## Representation

A value at the standard money scale — `Decimal< MONEY_SCALE >`. The name a
caller uses for money at the standard scale. The family's `money_*` functions
no longer name it: since
[`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)
each takes `Decimal< S >`, and a `Money` reaches it as `S = MONEY_SCALE`.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:58`

```rust
pub type Money = Decimal< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 58 | Declaration |
| `src/lib.rs` | 630-660,686-696 | Named by the `compile_fail` doctests: money and a quantity, and money and a price, do not mix |
| `tests/checked_arithmetic_test.rs`, `tests/parse_render_test.rs` | throughout | Concrete type exercised by this crate's own test suite |
| `tests/price_test.rs` | 6,8,13,21,45 | `Price`'s range checked against `Money`'s |
| `tests/rescale_test.rs` | 4,12,19,22,30,48 | The scale-6 side of every `rescale` |
| `exact_parse/src/lib.rs:39,46` | — | Compile-time assert that `Money::ONE_MINOR` matches `exact_scale::MONEY_SCALE` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export (`pub use exact_kind::{ .., Money, .. }`) |
| `smoke_exact_market_split/src/lib.rs` | 51,65,67,68,101,142,270,277,281,311,331,333 | The demo lane's money values, through `exact_arith` — steps 1 to 5; its scenes and step 7 run at other scales |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests |
| `exact_parse` | `src/lib.rs` | The cross-crate scale-consistency assert |
| `exact_arith` | `src/lib.rs` | Re-export |
| `smoke_exact_market_split` | `src/lib.rs` | Money at the standard scale, through `exact_arith` |
