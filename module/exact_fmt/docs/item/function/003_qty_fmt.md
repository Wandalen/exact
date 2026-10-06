# 003: qty_fmt

## Representation

Render a quantity to an owned `String`, through `Qty`'s own `Display` impl.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_fmt/src/lib.rs:112-115`

```rust
pub fn qty_fmt( v : Quantity ) -> String
{
  v.to_string()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 112-115 | Declaration |
| `tests/fmt_test.rs:15` | — | Matches a literal `"3"` |
| `tests/fmt_test.rs:81` | — | A fraction trimmed, and zero as plain `0` |
| `exact_arith/src/lib.rs:118` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Exercised by its own test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test suite |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `exact_kind::Qty::to_string` (via the blanket `ToString`
  impl every `Display` type gets)
