# 001: Decimal

## Representation

A fixed-point decimal carrying its scale in its type. `SCALE` is the number
of decimal places; the stored integer counts minor units, so the denoted
value is `minor × 10⁻ˢᶜᵃˡᵉ`. Ported from `exact_decimal`'s `Decimal< const
SCALE : u32 >` without behavioural change — only the backing alias
(`exact_minor::Backing`) and scale constants (`exact_scale`) now come from
sibling Tier-0 crates rather than being declared again here.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:153`

```rust
pub struct Decimal< const SCALE : u32 >
{
  minor : Backing,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 56,62,144-151,153,158,174,177,184,200,212,219,231,238,246,253,263,270,281,288,302,357,361,397,399,418,420,422,435,449,462,464 | Backs `Money`/`Price` type aliases; receiver/return type of every `Decimal` method; backs `Qty`'s `value` field |
| `tests/*.rs` (3 files) | throughout | Direct `Decimal::< N >::parse`/`::from_minor` construction alongside `Money` |
| 9 downstream crates (`exact_parse` … `exact_arith`) | `src/lib.rs` | Named only through the `Money`/`Price` aliases — see [Money](../type_alias/001_money.md), [Price](../type_alias/002_price.md) for the full per-crate breakdown; no downstream file names bare `Decimal` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The signed fixed-point representation `Money`/`Price` alias and `Qty` wraps |
| 9 downstream crates | — | Indirectly, exclusively through the `Money`/`Price` aliases (§ File Usage) |
