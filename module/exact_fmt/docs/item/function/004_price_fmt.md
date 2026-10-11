# 004: price_fmt

## Representation

Render a price value to an owned `String`, through `Price`'s own `Display`
impl, which renders the `Money` it wraps.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_fmt/src/lib.rs:119-122`

```rust
pub fn price_fmt< const S : u32 >( v : PriceAt< S > ) -> String
{
  v.to_string()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 119-122 | Declaration |
| `tests/fmt_test.rs:48` | — | A positive and a negative price render exactly |
| `exact_arith/src/lib.rs:118` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:21` | — | Renders a price in cents |

No caller outside this crate's own tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export; called at scale 2 by the any-scale facade test |

## Caller Tree

No production caller, intra-crate or external — an honest empty tree. Every
call site is in `exact_fmt`'s own tests; `exact_arith` only re-exports the
name.

## Callee Tree

- **External:** `exact_kind::Price::to_string` (via the blanket `ToString`
  impl every `Display` type gets; `Price`'s `Display` renders the wrapped `Money`)
