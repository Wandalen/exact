# 006: qty_div_round

## Representation

Divide a quantity by `d`, rounding the remainder per `rounding`. Unlike
`money_div_round`, a rounded result can legitimately land below zero —
`Quantity` refuses that, surfacing `RatioError::Negative`.

**Untested.** Verified via grep (`grep -rn qty_div_round` across the whole
`module/` tree): the matches are this declaration, the `exact_arith`
facade re-export, and one doc-comment mention in `exact_dust/src/lib.rs:26`
(prose explaining why `exact_dust` has no `price_dust_split`, not a call).
Neither `exact_ratio`'s own test suite nor any real call site exercises it —
an honest empty finding, not an omission.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:199`

```rust
pub fn qty_div_round( v : Quantity, d : i64, rounding : Rounding ) -> Result< Quantity, RatioError >
{
  let minor = div_round_minor( v.minor(), d, rounding )?;
  Quantity::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 199-203 | Declaration |
| `exact_arith/src/lib.rs:96` | — | Facade re-export |

No test file anywhere calls `qty_div_round`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Declared, but not exercised by any test of its own |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, production or test — an honest
empty tree, alongside [price_mul_ratio](004_price_mul_ratio.md) the other
genuinely unexercised function in this crate.

## Callee Tree

- `div_round_minor` (`src/lib.rs:171`, private — no Item Instance of its own)
- `kind_error_to_ratio_error` (`src/lib.rs:67`, private — no Item Instance of its own)
- **External:** `exact_kind::Quantity::minor`, `exact_kind::Quantity::from_minor`
