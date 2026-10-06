# 004: impl Display for Decimal

## Representation

Renders exactly, with trailing fractional zeros trimmed — a sign (if
negative), the whole-unit part, then `.` and the fractional digits with
trailing zeros stripped (omitted entirely when `SCALE == 0` or the fraction
is zero). The inverse of `parse` for every value it produces.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:373`

```rust
impl< const SCALE : u32 > fmt::Display for Decimal< SCALE >
{
  /// Render exactly, with trailing fractional zeros trimmed.
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    let unit = Self::ONE_MINOR;
    let magnitude = self.minor().unsigned_abs();
    let unit_u = unit.unsigned_abs();
    let whole = magnitude / unit_u;
    let frac = magnitude % unit_u;

    if self.minor() < 0
    {
      write!( f, "-" )?;
    }
    write!( f, "{whole}" )?;

    if SCALE == 0 || frac == 0
    {
      return Ok( () );
    }
    // Fix(exact_kind_display_allocated_per_render): the fraction was padded
    // into a `String` with `format!` and then trimmed — one heap allocation
    // per render, against feature 016's non-allocating display. The trailing
    // zeros are now counted arithmetically and the digits written directly.
    //
    // Root cause: `format!` used as a scratch buffer inside `fmt`.
    // Pitfall: `write!` into the formatter does not allocate but `format!`
    //   does, and the rendered text is identical — output tests cannot tell.
    let trailing_zeros = ( 1..=SCALE )
    .take_while( | &k | frac.is_multiple_of( pow10( k ).unsigned_abs() ) )
    .count();
    let digits = frac / pow10( trailing_zeros as u32 ).unsigned_abs();
    write!( f, ".{digits:0width$}", width = SCALE as usize - trailing_zeros )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 373-408 | Declaration |
| `tests/parse_render_test.rs` | throughout | Round-trip parse/render checks |
| `exact_fmt/src/lib.rs:107,121` | — | `money_fmt`/`price_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised extensively by its own round-trip tests |
| `exact_fmt` | `src/lib.rs` | `money_fmt`/`price_fmt`'s entire implementation is this `Display` via `.to_string()` |
