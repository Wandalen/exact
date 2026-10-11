# 003: Quantity

## Representation

A non-negative quantity at the standard money scale — `Qty< MONEY_SCALE >`.
The name a caller uses for a quantity at the standard scale; the family's
`qty_*` functions take `Qty< S >` instead (→
[`exact_arith`'s ADR-001](../../../../exact_arith/docs/decisions/001_operations_generic_over_scale.md)). Mirrors
[`Money`](001_money.md) for the non-negative wrapper type instead of the
signed one.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:61`

```rust
pub type Quantity = Qty< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 61,446-449 | Declaration; struct-doc-comment doctest on `Qty` |
| `src/lib.rs` | 615-644 | Named by the `compile_fail` doctests: money and a quantity do not mix |
| `tests/non_negative_test.rs` | throughout | Concrete type exercised by this crate's own test suite |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |
| `smoke_exact_market_split/src/lib.rs` | 51,161,201,202,204,302,303 | The demo lane's quantities and lot size, through `exact_arith` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests and its own struct-doc doctest |
| `exact_arith` | `src/lib.rs` | Re-export |
| `smoke_exact_market_split` | `src/lib.rs` | Quantities at the standard scale, through `exact_arith` |
