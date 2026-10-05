# 001: money_from_str

## Representation

Parse a money value from text, dispatching straight to `exact_kind`'s own
parser.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_parse/src/lib.rs:52`

```rust
pub fn money_from_str( text : &str ) -> Result< Money, KindError >
{
  Money::parse( text )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 52 | Declaration |
| `tests/from_str_test.rs:12,28` | — | Exact round-trip through `"1.23"`; malformed-text rejection loop (as a function-pointer array element, not a direct call at that line — the actual call is via the `parser` variable at line 30) |
| `exact_arith/src/lib.rs:104` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Exercised by its own round-trip and malformed-text tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test, which calls `exact_kind::Decimal::parse` (via `Money::parse`) directly instead (see [readme](../readme.md) Notable Findings) |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, grep-verified
across every `.rs` file in the workspace. `exact_arith` only re-exports the
name; no production code anywhere calls `money_from_str` itself.

## Callee Tree

- **External:** `exact_kind::Decimal::parse` (via `Money::parse( text )`, `Money` being an alias for `Decimal< SCALE >`)
