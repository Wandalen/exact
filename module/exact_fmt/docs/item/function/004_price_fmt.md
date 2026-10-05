# 004: price_fmt

## Representation

Render a price value to an owned `String`, through `Price`'s own `Display`
impl, which renders the `Money` it wraps.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_fmt/src/lib.rs:119-122`

```rust
pub fn price_fmt( v : Price ) -> String
{
  v.to_string()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 119-122 | Declaration |
| `tests/fmt_test.rs:48` | — | A positive and a negative price render exactly |
| `exact_arith/src/lib.rs:106` | — | Facade re-export |

No caller outside this crate's own tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No production caller, intra-crate or external — an honest empty tree. Every
call site is in `exact_fmt`'s own tests; `exact_arith` only re-exports the
name.

## Callee Tree

- **External:** `exact_kind::Price::to_string` (via the blanket `ToString`
  impl every `Display` type gets; `Price`'s `Display` renders the wrapped `Money`)
