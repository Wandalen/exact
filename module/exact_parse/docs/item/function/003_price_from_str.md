# 003: price_from_str

## Representation

Parse a price value from text. As [money_from_str](001_money_from_str.md) —
`Price` is `Money` under today's disclosed deviation in `exact_kind` (both
alias the same `Decimal< SCALE >` instantiation).

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_parse/src/lib.rs:64`

```rust
pub fn price_from_str( text : &str ) -> Result< Price, KindError >
{
  Price::parse( text )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 64 | Declaration |
| `tests/from_str_test.rs:14,28` | — | Exact round-trip through `"1.23"`; malformed-text rejection loop (as a function-pointer array element, not a direct call at that line — the actual call is via the `parser` variable at line 30) |
| `exact_arith/src/lib.rs:98` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Exercised by its own round-trip and malformed-text tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test (see [readme](../readme.md) Notable Findings) |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, grep-verified
across every `.rs` file in the workspace.

## Callee Tree

- **External:** `exact_kind::Decimal::parse` (via `Price::parse( text )`, `Price` being an alias for `Decimal< SCALE >`)
