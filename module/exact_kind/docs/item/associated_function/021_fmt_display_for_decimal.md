# 021: Display::fmt for Decimal

## Representation

Renders exactly, with trailing fractional zeros trimmed — or, given a
precision (`{:.2}`), with exactly that many digits after the point — padded
to the formatter's width, fill, alignment, `+` and `0` as an integer is. The
whole body is one call to the private `render`; see
[impl Display for Decimal](../implementation/004_display_for_decimal.md).

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:526`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  render( self.minor(), SCALE, f )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 526-529 | Declaration |
| `tests/parse_render_test.rs` | throughout | Round-trip parse/render, called via `.to_string()` |
| `tests/display_precision_test.rs` | throughout | `{:.N}` and the width, fill, alignment, `+` and `0` flags, via `format!`, and against `f64`'s output |
| `exact_fmt/src/lib.rs:107,121` | — | `money_fmt`/`price_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised extensively by its own round-trip tests |
| `exact_fmt` | `src/lib.rs` | **Production** — the entire implementation of `money_fmt`/`price_fmt` is this `Display` invoked via `.to_string()` |

## Caller Tree

- **External:** `exact_fmt::money_fmt` (`exact_fmt/src/lib.rs:107`), `price_fmt` (`:121`) — both via `v.to_string()`, the standard library's blanket `ToString` bridging to this `Display` impl

- **Intra-crate:** [Display::fmt for Qty](022_fmt_display_for_qty.md) (`src/lib.rs:849`) and `Display::fmt` for `PriceAt` (`src/lib.rs:978`), each forwarding its own formatter so a precision reaches this one

## Callee Tree

- `render` (`src/lib.rs:546-642`, private — no Item Instance of its own), for every render, with or without a precision
  - **External:** `exact_round::round_div` under `Rounding::HalfEven`, when fewer places than `SCALE` are asked (`src/lib.rs:554`)
  - **External:** `fmt::Formatter::precision`, `width`, `fill`, `align`, `sign_plus`, `sign_aware_zero_pad` — the flags
  - **External:** `fmt::Write::write_char` (fill and sign), `fmt::Formatter::write_str` and `write_fmt` (via `write!`) — the digits, written straight in with no `String` allocated
