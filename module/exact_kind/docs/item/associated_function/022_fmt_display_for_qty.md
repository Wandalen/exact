# 022: Display::fmt for Qty

## Representation

Delegates entirely to the same count as a `Decimal` (the private `value()`) and its own rendering, passing the
formatter itself through, so a requested precision (`{:.2}`) reaches it.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:846`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  // Forward the formatter itself, not `"{}"`, so a precision reaches the decimal.
  fmt::Display::fmt( &self.value(), f )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 846-850 | Declaration |
| `tests/non_negative_test.rs:50` | — | Compared against the inner decimal's own rendering, via `.to_string()` |
| `tests/display_precision_test.rs` | — | `{:.2}`/`{:.3}`/`{:.4}` on a `Quantity` and a `Qty< 8 >` |
| `exact_fmt/src/lib.rs:114` | — | `qty_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised once, by its own delegation-equivalence test |
| `exact_fmt` | `src/lib.rs` | **Production** — the entire implementation of `qty_fmt` is this `Display` invoked via `.to_string()` |

## Caller Tree

- **External:** `exact_fmt::qty_fmt` (`exact_fmt/src/lib.rs:114`) — via `v.to_string()`

No intra-crate caller.

## Callee Tree

- [Display::fmt for Decimal](021_fmt_display_for_decimal.md) (`src/lib.rs:849`, via `fmt::Display::fmt( &self.value(), f )` — the same formatter, so its precision carries over)
