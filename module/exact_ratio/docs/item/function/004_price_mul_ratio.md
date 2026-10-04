# 004: price_mul_ratio

## Representation

Multiply a price by `n / d`, rounding the result per `rounding`. Its own doc comment states the error behavior
is "as `money_mul_ratio`" — the body is the same as
[money_mul_ratio](002_money_mul_ratio.md) with `Price` in place of `Money`.

**Untested.** Verified via grep (`grep -rn price_mul_ratio` across the whole
`module/` tree): the only matches are this declaration and the
`exact_arith` facade re-export. Neither `exact_ratio`'s own test suite nor
any other crate calls it — an honest empty finding, not an omission. This
mirrors `exact_ratio`'s own module doc comment's disclosure
(`exact_dust/src/lib.rs:25-26`) that the sibling crate `exact_dust`
deliberately ships no `price_dust_split` for the same reason: "splitting a
*price* into equal shares has no natural reading and no consumer anywhere in
this codebase."

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:165`

```rust
pub fn price_mul_ratio( v : Price, r : Ratio, rounding : Rounding ) -> Result< Price, RatioError >
{
  let minor = mul_ratio_minor( v.minor(), r, rounding )?;
  Price::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 165-169 | Declaration |
| `exact_arith/src/lib.rs:102` | — | Facade re-export |

No test file anywhere calls `price_mul_ratio`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Declared, but not exercised by any test of its own |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, production or test — an honest
empty tree. The sharpest finding in this crate: the function is public,
re-exported, and documented, but has zero verified exercise anywhere in the
workspace today.

## Callee Tree

- `mul_ratio_minor` (`src/lib.rs:127`, private — no Item Instance of its own)
- `kind_error_to_ratio_error` (`src/lib.rs:67`, private — no Item Instance of its own)
- **External:** `exact_kind::Price::minor`, `exact_kind::Price::from_minor` (each delegating to `Decimal`'s)
