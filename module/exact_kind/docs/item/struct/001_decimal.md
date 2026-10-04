# 001: Decimal

## Representation

A fixed-point decimal carrying its scale in its type. `SCALE` is the number
of decimal places; the stored integer counts minor units, so the denoted
value is `minor × 10⁻ˢᶜᵃˡᵉ`. Ported from `exact_decimal`'s `Decimal< const
SCALE : u32 >` without behavioural change. What it is built on now comes
from the sibling Tier-0 crates rather than being declared again here: the
stored count is an `exact_minor::Minor`, added, subtracted and negated by
`exact_minor`'s checked functions, and the scale constants come from
`exact_scale`.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:157`

```rust
pub struct Decimal< const SCALE : u32 >
{
  minor : Minor,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 57,634,139-146,157,173,189,192,199,215,227,234,246,250,260,264,276,283,294,298,314,369,373,412,414,433,435,437,450,464,477,479 | Backs the `Money` alias and `Price`'s `value` field; receiver/return type of every `Decimal` method; backs `Qty`'s `value` field |
| `tests/*.rs` (3 files) | throughout | Direct `Decimal::< N >::parse`/`::from_minor` construction alongside `Money` |
| 9 downstream crates (`exact_parse` … `exact_arith`) | `src/lib.rs` | Named only through the `Money` alias and `Price` — see [Money](../type_alias/001_money.md), [Price](003_price.md) for the full per-crate breakdown; no downstream file names bare `Decimal` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The signed fixed-point representation `Money` aliases and `Qty` and `Price` wrap |
| 9 downstream crates | — | Indirectly, exclusively through `Money` and `Price` (§ File Usage) |
