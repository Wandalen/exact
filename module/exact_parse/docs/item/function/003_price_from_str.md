# 003: price_from_str

## Representation

Parse a price value from text. As [money_from_str](001_money_from_str.md) —
`Price::parse` delegates to the same `Decimal< SCALE >` parser and wraps the
result as a `Price`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_parse/src/lib.rs:73`

```rust
pub fn price_from_str< const S : u32 >( text : &str ) -> Result< PriceAt< S >, KindError >
{
  PriceAt::< S >::parse( text )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 73 | Declaration |
| `tests/from_str_test.rs:15,31-32` | — | Exact round-trip through `"1.23"`; malformed text (`"NaN"`, `""`) refused, each by a direct call |
| `exact_arith/src/lib.rs:117` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:23` | — | Parses a price at scale 2, named by turbofish |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Exercised by its own round-trip and malformed-text tests |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export; called at scale 2 by the any-scale facade test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, grep-verified
across every `.rs` file in the workspace.

## Callee Tree

- **External:** `exact_kind::Price::parse` (via `Price::parse( text )`), which delegates to `exact_kind::Decimal::parse`
