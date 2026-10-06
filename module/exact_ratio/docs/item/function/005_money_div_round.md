# 005: money_div_round

## Representation

Divide a money value by `d`, rounding the remainder per `rounding`. The
family's one shared sign-handling, tie-breaking division logic
(`exact_round::round_div`) is driven here rather than duplicated.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:206`

```rust
pub fn money_div_round( v : Money, d : i64, rounding : Rounding ) -> Result< Money, RatioError >
{
  let minor = div_round_minor( v.minor(), d, rounding )?;
  Money::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 206-210 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 82-83,92-93,102,106,110,119,123,131,141 | Every rounding mode (`Down`/`Up`/`HalfEven`), both signs, tie and non-tie remainders, zero-divisor refusal, and exact-division agreement across modes — by far the most heavily tested function in this crate |
| `exact_arith/src/lib.rs:107` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised exhaustively by its own test suite |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name; no other workspace crate depends on
`exact_ratio` today. Notably, `exact_dust` — which also needs mode-driven
integer division — depends on `exact_round` directly rather than reaching it
through this function (`exact_dust/src/lib.rs:12-19`, a disclosed deviation
from the plan's own Tier 3 dependency table, which lists `exact_dust` as
depending on `exact_kind, exact_ratio`).

## Callee Tree

- `div_round_minor` (`src/lib.rs:191`, private — no Item Instance of its own)
- `kind_error_to_ratio_error` (`src/lib.rs:80`, private — no Item Instance of its own)
- **External:** `exact_kind::Money::minor`, `exact_kind::Money::from_minor`
