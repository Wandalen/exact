# 004: price_mul_ratio

## Representation

Multiply a price by `n / d`, rounding the result per `rounding`. Its own doc comment states the error behavior
is "as `money_mul_ratio`" — the body is the same as
[money_mul_ratio](002_money_mul_ratio.md) with `Price` in place of `Money`.

**No production caller.** Verified via grep (`grep -rn price_mul_ratio`
across the whole `module/` tree): outside this crate's own tests, the only
matches are this declaration and the `exact_arith` facade re-export — no
other crate calls it. This mirrors `exact_ratio`'s own module doc comment's disclosure
(`exact_dust/src/lib.rs:25-26`) that the sibling crate `exact_dust`
deliberately ships no `price_dust_split` for the same reason: "splitting a
*price* into equal shares has no natural reading and no consumer anywhere in
this codebase."

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:203`

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
| `src/lib.rs` | 203-207 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 231,246 | `Down`/`Up`/`HalfEven` at both signs, and refusal past the ceiling |
| `exact_arith/src/lib.rs:110` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No production caller, intra-crate or external — an honest empty tree. Every
call site is in `exact_ratio`'s own tests, which are out of scope for this
tree (production call-graph only); `exact_arith` only re-exports the name.

## Callee Tree

- `mul_ratio_minor` (`src/lib.rs:155`, private — no Item Instance of its own)
  - `round_error_to_ratio_error` (`src/lib.rs:119`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div_wide`'s result)
- `kind_error_to_ratio_error` (`src/lib.rs:84`, private — no Item Instance of its own)
- **External:** `exact_kind::Price::minor`, `exact_kind::Price::from_minor` (each delegating to `Decimal`'s)
