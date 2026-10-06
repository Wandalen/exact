# 001: Decimal::ONE_MINOR

## Representation

One whole unit, in minor units — `pow10( SCALE )`. The scaling factor every
`from_int`/`Display` computation is built from.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:176`

```rust
pub const ONE_MINOR : Backing = pow10( SCALE );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 176,217,236,364-365,378 | Declaration; `from_int`'s scaling multiply; `whole`'s divide; `parse`'s magnitude accumulation; `Display`'s whole/frac split |
| `exact_parse/src/lib.rs:45` | — | **Production** — compile-time assert cross-checking `Money::ONE_MINOR` against `exact_scale::pow10( MONEY_SCALE )` directly |
| `exact_ratio/src/lib.rs:238,245` | — | **Production** — `price_mul_qty`'s quantity-as-ratio denominator, and the compile-time assert that `Quantity` and `Money` share that scale |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The scaling factor for every whole-unit conversion and the renderer |
| `exact_parse` | `src/lib.rs` | A compile-time consistency check between `exact_kind` and `exact_scale`'s own constants — not consumed for any runtime computation |
| `exact_ratio` | `src/lib.rs` | `price_mul_qty` counts one whole quantity as `Money::ONE_MINOR` minor units |
