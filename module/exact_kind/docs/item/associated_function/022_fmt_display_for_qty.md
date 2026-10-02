# 022: Display::fmt for Qty

## Representation

Delegates entirely to the wrapped `Decimal`'s own rendering.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:539`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  write!( f, "{}", self.value )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 539-542 | Declaration |
| `tests/non_negative_test.rs:50` | — | Compared against the inner decimal's own rendering, via `.to_string()` |
| `exact_fmt/src/lib.rs:100` | — | `qty_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised once, by its own delegation-equivalence test |
| `exact_fmt` | `src/lib.rs` | **Production** — the entire implementation of `qty_fmt` is this `Display` invoked via `.to_string()` |

## Caller Tree

- **External:** `exact_fmt::qty_fmt` (`exact_fmt/src/lib.rs:100`) — via `v.to_string()`

No intra-crate caller.

## Callee Tree

- [Display::fmt for Decimal](021_fmt_display_for_decimal.md) (`src/lib.rs:541`, via `write!( f, "{}", self.value )`)
