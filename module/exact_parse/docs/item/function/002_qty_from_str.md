# 002: qty_from_str

## Representation

Parse a quantity from text, refusing a negative one. Dispatches straight to
`exact_kind`'s own parser, which carries the non-negativity refusal.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_parse/src/lib.rs:53`

```rust
pub fn qty_from_str( text : &str ) -> Result< Quantity, KindError >
{
  Quantity::parse( text )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 53 | Declaration |
| `tests/from_str_test.rs:13,21,32` | — | Exact round-trip through `"1.23"`; negative-value refusal (`KindError::Negative`); `"NaN"` rejection |
| `exact_arith/src/lib.rs:104` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | Exercised by its own round-trip, negative-refusal, and malformed-text tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test (see [readme](../readme.md) Notable Findings) |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, grep-verified
across every `.rs` file in the workspace.

## Callee Tree

- **External:** `exact_kind::Qty::parse` (via `Quantity::parse( text )`, `Quantity` being an alias for `Qty< SCALE >`)
