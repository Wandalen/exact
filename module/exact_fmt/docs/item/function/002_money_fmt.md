# 002: money_fmt

## Representation

Render a money value to an owned `String`, through `Money`'s own `Display`
impl.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_fmt/src/lib.rs:105-108`

```rust
pub fn money_fmt( v : Money ) -> String
{
  v.to_string()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 105-108 | Declaration |
| `tests/fmt_test.rs:11-12` | — | Matches `v.to_string()` directly and a literal `"1.5"` |
| `tests/fmt_test.rs:89` | — | The smallest negative amount keeps every leading fractional zero |
| `exact_arith/src/lib.rs:118` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Exercised by its own test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test suite |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `exact_kind::Decimal::to_string` (via the blanket `ToString`
  impl every `Display` type gets; `Money` is an alias for `Decimal< SCALE >`)
