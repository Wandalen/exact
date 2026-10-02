# 004: impl Display for Decimal

## Representation

Renders exactly, with trailing fractional zeros trimmed — a sign (if
negative), the whole-unit part, then `.` and the fractional digits with
trailing zeros stripped (omitted entirely when `SCALE == 0` or the fraction
is zero). The inverse of `parse` for every value it produces.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:361`

```rust
impl< const SCALE : u32 > fmt::Display for Decimal< SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    let unit = Self::ONE_MINOR;
    let magnitude = self.minor.unsigned_abs();
    let unit_u = unit.unsigned_abs();
    let whole = magnitude / unit_u;
    let frac = magnitude % unit_u;

    if self.minor < 0
    {
      write!( f, "-" )?;
    }
    write!( f, "{whole}" )?;

    if SCALE == 0 || frac == 0
    {
      return Ok( () );
    }
    let padded = format!( "{frac:0width$}", width = SCALE as usize );
    write!( f, ".{}", padded.trim_end_matches( '0' ) )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 361-385 | Declaration |
| `tests/parse_render_test.rs` | throughout | Round-trip parse/render checks |
| `exact_fmt/src/lib.rs:93,107` | — | `money_fmt`/`price_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised extensively by its own round-trip tests |
| `exact_fmt` | `src/lib.rs` | `money_fmt`/`price_fmt`'s entire implementation is this `Display` via `.to_string()` |
