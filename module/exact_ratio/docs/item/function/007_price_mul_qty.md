# 007: price_mul_qty

## Representation

The money a trade costs: `price × qty`, rounded per `rounding`. A quantity is
itself a ratio — its minor-unit count over one whole unit — so this reuses
`mul_ratio_minor`'s widened multiply and rounded divide with the quantity as
the ratio, and a fractional quantity counts in full. It replaces the pattern
`price.checked_mul_int( qty.whole() )`, which silently drops the quantity's
fractional part (`1.25 × 4.5` came out as `5`, not `5.625`).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:227`

```rust
pub fn price_mul_qty( price : Price, qty : Quantity, rounding : Rounding ) -> Result< Money, RatioError >
{
  // `Quantity` and `Money` share one scale, so one whole quantity is `Money::ONE_MINOR` minor units.
  let qty_as_ratio = ratio_new( qty.minor(), Money::ONE_MINOR )?;
  let minor = mul_ratio_minor( price.minor(), qty_as_ratio, rounding )?;
  Money::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 227-233 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 161,170,182,303 | A fractional quantity; a cost finer than one minor unit under each mode; a cost past the ceiling; a negative price and a zero quantity |
| `exact_arith/src/lib.rs:102` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:25` | — | The settlement test's notional, through the facade |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; its settlement test computes the trade's notional with it |

## Caller Tree

- **External:** `exact_arith`'s own test, `facade_test.rs:25` (test-context, via the re-exported name)

No production caller inside this repository yet.

## Callee Tree

- [ratio_new](001_ratio_new.md) — the quantity as a ratio over one whole unit
- `mul_ratio_minor` (private — no Item Instance of its own) — widen, multiply, round, narrow
- **External:** `exact_kind::Decimal::from_minor` — the declared-ceiling check on the cost
