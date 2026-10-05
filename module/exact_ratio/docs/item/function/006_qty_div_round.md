# 006: qty_div_round

## Representation

Divide a quantity by `d`, rounding the remainder per `rounding`. Unlike
`money_div_round`, a rounded result can legitimately land below zero —
`Quantity` refuses that, surfacing `RatioError::Negative`.

**No production caller.** Verified via grep (`grep -rn qty_div_round` across the whole
`module/` tree): the matches are this declaration, the `exact_arith`
facade re-export, and one doc-comment mention in `exact_dust/src/lib.rs:26`
(prose explaining why `exact_dust` has no `price_dust_split`, not a call).
No real call site outside this crate's own tests exercises it.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:211`

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
| `src/lib.rs` | 211-215 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 271,281,291 | Every mode, a zero divisor, and a negative divisor — refused unless the result rounds to zero |
| `exact_arith/src/lib.rs:102` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No production caller, intra-crate or external — an honest empty tree. Every
call site is in `exact_ratio`'s own tests; `exact_arith` only re-exports the
name.

## Callee Tree

- `div_round_minor` (`src/lib.rs:183`, private — no Item Instance of its own)
- `kind_error_to_ratio_error` (`src/lib.rs:80`, private — no Item Instance of its own)
- **External:** `exact_kind::Quantity::minor`, `exact_kind::Quantity::from_minor`
