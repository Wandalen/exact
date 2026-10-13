# 001: Decimal::ONE_MINOR

## Representation

One whole unit, in minor units — `pow10( SCALE )`. The scaling factor every
`from_int`/`whole`/`parse` computation is built from, and the value the
scale check `REPRESENTABLE` tests. `Display` renders from the scale itself.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:180`

```rust
pub const ONE_MINOR : Backing = pow10( SCALE );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 180,185,227,284,412-413 | Declaration; the `REPRESENTABLE` scale check; `from_int`'s scaling multiply; `whole`'s divide; `parse`'s magnitude accumulation |
| `exact_parse/src/lib.rs:46` | — | **Production** — compile-time assert cross-checking `Money::ONE_MINOR` against `exact_scale::pow10( MONEY_SCALE )` directly |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The scaling factor for every whole-unit conversion, and the scale check |
| `exact_parse` | `src/lib.rs` | A compile-time consistency check between `exact_kind` and `exact_scale`'s own constants — not consumed for any runtime computation |
