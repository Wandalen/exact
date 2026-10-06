# 003: price_from_str

## Representation

Parse a price value from text. As [money_from_str](001_money_from_str.md) —
`Price::parse` delegates to the same `Decimal< SCALE >` parser and wraps the
result as a `Price`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_parse/src/lib.rs:72`

```rust
pub fn price_from_str( text : &str ) -> Result< Price, KindError >
{
  Price::parse( text )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 72 | Declaration |
| `tests/from_str_test.rs:14,30-31` | — | Exact round-trip through `"1.23"`; malformed text (`"NaN"`, `""`) refused, each by a direct call |
| `exact_arith/src/lib.rs:116` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Exercised by its own round-trip and malformed-text tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test (see [readme](../readme.md) Notable Findings) |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, grep-verified
across every `.rs` file in the workspace.

## Callee Tree

- **External:** `exact_kind::Price::parse` (via `Price::parse( text )`), which delegates to `exact_kind::Decimal::parse`
