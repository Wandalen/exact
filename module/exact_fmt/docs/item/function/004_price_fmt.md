# 004: price_fmt

## Representation

Render a price value to an owned `String`, through `Price`'s own `Display`
impl, which renders the `Money` it wraps.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_fmt/src/lib.rs:105-108`

```rust
pub fn price_fmt( v : Price ) -> String
{
  v.to_string()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 105-108 | Declaration |
| `exact_arith/src/lib.rs:106` | — | Facade re-export |

No test file anywhere calls `price_fmt` — not even this crate's own
`tests/fmt_test.rs`, which imports and exercises `money_fmt`/`qty_fmt` but
not this one (confirmed: its `use` statement at line 3 names `FmtError`,
`fmt_into`, `money_fmt`, `qty_fmt` only). The sharpest finding in this crate:
a public function with zero callers, intra-crate or external, that isn't
even reached by its own defining crate's test suite.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, and the
only function in this crate untested even by its own defining crate.

## Callee Tree

- **External:** `exact_kind::Price::to_string` (via the blanket `ToString`
  impl every `Display` type gets; `Price`'s `Display` renders the wrapped `Money`)
