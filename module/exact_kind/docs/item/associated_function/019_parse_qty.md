# 019: Qty::parse

## Representation

Parse a decimal string, refusing a negative one. Delegates entirely to
`Decimal::parse` via the `?` operator, then routes the result through
`from_decimal` for the non-negativity check.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:554`

```rust
pub fn parse( text : &str ) -> Result< Self, KindError >
{
  Self::from_decimal( Decimal::parse( text )? )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 554 | Declaration |
| `tests/non_negative_test.rs` | throughout | Negative-string refusal, `"-0.0"` edge case |
| `exact_parse/src/lib.rs:62,64` | — | `qty_from_str`'s entire body |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own tests |
| `exact_parse` | `src/lib.rs` | **Production** — the entire implementation of `qty_from_str` |

## Caller Tree

- **External:** `exact_parse::qty_from_str` (`exact_parse/src/lib.rs:64`)

No intra-crate caller.

## Callee Tree

- [Decimal::parse](009_parse_decimal.md) (`src/lib.rs:556`, via the `?` operator)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:556`)
