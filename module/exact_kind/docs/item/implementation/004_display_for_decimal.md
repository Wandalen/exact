# 004: impl Display for Decimal

## Representation

Renders exactly, with trailing fractional zeros trimmed — a sign (if
negative), the whole-unit part, then `.` and the fractional digits with
trailing zeros stripped (omitted entirely when `SCALE == 0` or the fraction
is zero). The inverse of `parse` for every value it produces.

When the caller asks for a precision — `{:.2}` — it prints exactly that many
digits after the point instead: extra places are zeros, fewer round half to
even. Either way the text is then padded to the formatter's width, fill and
alignment, with `+` and `0` honoured, the way an integer is: `{:>10.2}` of
`1.5` is `      1.50`. All of it is the private `render` (`src/lib.rs:450-546`),
which writes straight into the formatter: only the text is rounded, never
the value, and nothing is allocated
(→ [ADR-004](../../decisions/004_display_precision_rounds_half_to_even.md)).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:421-434`

```rust
impl< const SCALE : u32 > fmt::Display for Decimal< SCALE >
{
  /// Render exactly, with trailing fractional zeros trimmed — or, when the
  /// caller asks for a precision (`{:.2}`), with exactly that many digits
  /// after the point. Width, fill, alignment, `+` and `0` are honoured the way
  /// they are for integers: `{:>10.2}` of `1.5` is `      1.50`. A precision
  /// past the scale pads zeros that `parse` at this scale refuses: `{:.4}` of a
  /// `Decimal< 2 >` `1.5` is `1.5000`, which `Decimal::< 2 >::parse` rejects as
  /// `ExcessPrecision`.
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    render( self.minor(), SCALE, f )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 421-546 | Declaration |
| `tests/parse_render_test.rs` | throughout | Round-trip parse/render checks |
| `tests/display_precision_test.rs` | throughout | `{:.N}`: padding, half-even rounding, negatives, `Qty`/`PriceAt` pass-through; width, fill, alignment, `+` and `0`, and every flag against `f64`'s output |
| `exact_arith/tests/no_alloc_test.rs` | — | `{:.N}`, width, fill and `+` rendered into a stack buffer with no allocation |
| `smoke_exact_market_split/src/lib.rs:444-445` | — | Step 7 prints its amounts in cents with `{:.2}` |
| `exact_fmt/src/lib.rs:107,121` | — | `money_fmt`/`price_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised extensively by its own round-trip tests |
| `exact_fmt` | `src/lib.rs` | `money_fmt`/`price_fmt`'s entire implementation is this `Display` via `.to_string()` |
