# 021: Display::fmt for Decimal

## Representation

Renders exactly, with trailing fractional zeros trimmed. See
[impl Display for Decimal](../implementation/004_display_for_decimal.md) for
the full body.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:364`

```rust
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
  let trailing_zeros = ( 1..=SCALE ).take_while( | &k | frac.is_multiple_of( 10_u64.pow( k ) ) ).count();
  write!( f, ".{:0width$}", frac / 10_u64.pow( trailing_zeros as u32 ), width = SCALE as usize - trailing_zeros )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 364-384 | Declaration |
| `tests/parse_render_test.rs` | throughout | Round-trip parse/render, called via `.to_string()` |
| `exact_fmt/src/lib.rs:93,107` | — | `money_fmt`/`price_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised extensively by its own round-trip tests |
| `exact_fmt` | `src/lib.rs` | **Production** — the entire implementation of `money_fmt`/`price_fmt` is this `Display` invoked via `.to_string()` |

## Caller Tree

- **External:** `exact_fmt::money_fmt` (`exact_fmt/src/lib.rs:93`), `price_fmt` (`:107`) — both via `v.to_string()`, the standard library's blanket `ToString` bridging to this `Display` impl

No intra-crate caller (this crate's own tests invoke it only via
`.to_string()` in test-context, out of Caller Tree scope per OT012).

## Callee Tree

- **External:** `i64::unsigned_abs` (×2 — magnitude and unit)
- **External:** `fmt::Formatter::write_fmt` (via the three `write!` calls)
- **External:** `u64::pow` — counting and removing trailing fractional zeros arithmetically, with no `String` allocated
