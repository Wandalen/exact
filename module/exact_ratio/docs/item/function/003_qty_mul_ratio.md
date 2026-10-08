# 003: qty_mul_ratio

## Representation

Multiply a quantity by `n / d`, rounding the result per `rounding`. Unlike `money_mul_ratio`, a negative-numerator
ratio can legitimately take the result below zero — `Quantity` refuses that,
surfacing `RatioError::Negative` rather than wrapping or silently clamping.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:175`

```rust
pub fn qty_mul_ratio( v : Quantity, r : Ratio, rounding : Rounding ) -> Result< Quantity, RatioError >
{
  let minor = mul_ratio_minor( v.minor(), r, rounding )?;
  Quantity::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 175-179 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 60-67,279,350 | Negative-numerator ratio refused as `RatioError::Negative`; `Down`/`Up`/`HalfEven` at both signs; a negative product within one minor unit of zero decided by each of the seven modes, and a whole-unit one refused in every mode |
| `exact_arith/src/lib.rs:112` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name; no other workspace crate depends on
`exact_ratio` today.

## Callee Tree

- `mul_ratio_minor` (`src/lib.rs:141`, private — no Item Instance of its own)
- `kind_error_to_ratio_error` (`src/lib.rs:80`, private — no Item Instance of its own)
- **External:** `exact_kind::Quantity::minor`, `exact_kind::Quantity::from_minor`
