# 002: money_mul_ratio

## Representation

Multiply a money value by `n / d`, rounding the result per `rounding`. Widens to `i128` before dividing, so an
intermediate product that would overflow `i64` still succeeds as long as the
final result fits.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_ratio/src/lib.rs:161`

```rust
pub fn money_mul_ratio( v : Money, r : Ratio, rounding : Rounding ) -> Result< Money, RatioError >
{
  let minor = mul_ratio_minor( v.minor(), r, rounding )?;
  Money::from_minor( minor ).map_err( kind_error_to_ratio_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 161-165 | Declaration |
| `tests/ratio_and_div_round_test.rs` | 34,44 | One-half exact multiply; an intermediate-overflow survival case |
| `exact_arith/src/lib.rs:108` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.
`exact_arith` only re-exports the name; no other workspace crate depends on
`exact_ratio` today (confirmed via `grep -rl exact_ratio --include=Cargo.toml`
across `substrate/` and `module/`).

## Callee Tree

- `mul_ratio_minor` (`src/lib.rs:140`, private — no Item Instance of its own)
- `kind_error_to_ratio_error` (`src/lib.rs:80`, private — no Item Instance of its own)
- **External:** `exact_kind::Money::minor`, `exact_kind::Money::from_minor`
