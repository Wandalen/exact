# 009: Decimal::parse

## Representation

Parse a decimal string exactly, or say why it cannot be. An optional sign, at
least one integer digit, and an optional fractional part of at most `SCALE`
digits — total and narrow, rejecting rather than repairing anything outside
the grammar. The sole text-to-value entry point for the type.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:302`

```rust
pub fn parse( text : &str ) -> Result< Self, KindError >
{
  let ( negative, digits ) = match text.strip_prefix( '-' )
  {
    Some( rest ) => ( true, rest ),
    None => ( false, text.strip_prefix( '+' ).unwrap_or( text ) ),
  };

  let ( int_part, frac_part ) = match digits.split_once( '.' )
  {
    Some( ( _, "" ) ) => return Err( KindError::Malformed { reason : "no digit after the decimal point" } ),
    Some( ( i, f ) ) => ( i, f ),
    None => ( digits, "" ),
  };

  if int_part.is_empty()
  {
    return Err( KindError::Malformed { reason : "no integer digit" } );
  }
  if !int_part.bytes().all( | b | b.is_ascii_digit() )
  {
    return Err( KindError::Malformed { reason : "non-digit in integer part" } );
  }
  if !frac_part.bytes().all( | b | b.is_ascii_digit() )
  {
    return Err( KindError::Malformed { reason : "non-digit in fractional part" } );
  }

  let supplied = u32::try_from( frac_part.len() ).unwrap_or( u32::MAX );
  if supplied > SCALE
  {
    return Err( KindError::ExcessPrecision { digits : supplied, scale : SCALE } );
  }

  let whole : Backing = int_part
  .parse()
  .map_err( | _ | KindError::Overflow { operation : "parse" } )?;

  let frac : Backing = if frac_part.is_empty()
  {
    0
  }
  else
  {
    frac_part
    .parse::< Backing >()
    .map_err( | _ | KindError::Overflow { operation : "parse" } )?
    * pow10( SCALE - supplied )
  };

  let magnitude = whole
  .checked_mul( Self::ONE_MINOR )
  .and_then( | m | m.checked_add( frac ) )
  .ok_or( KindError::Overflow { operation : "parse" } )?;

  Self::from_minor( if negative { -magnitude } else { magnitude } )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 302,533 | Declaration; `Qty::parse`'s delegation via `?` |
| `tests/parse_render_test.rs`, `tests/checked_arithmetic_test.rs` | throughout | Grammar acceptance/rejection, round-trip |
| `exact_parse/src/lib.rs:45,66` | — | `money_from_str`, `price_from_str` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::parse`; exercised extensively by its own tests |
| `exact_parse` | `src/lib.rs` | **Production** — the entire implementation of `money_from_str`/`price_from_str` |

## Caller Tree

- [Qty::parse](019_parse_qty.md) (`src/lib.rs:533` — via the `?` operator)
- **External:** `exact_parse::money_from_str` (`exact_parse/src/lib.rs:45`), `price_from_str` (`:66`)

## Callee Tree

- **External:** `str::strip_prefix` (×2 — leading sign)
- **External:** `str::split_once` — splitting on `.`
- **External:** `u8::is_ascii_digit` via `.bytes().all( .. )` (×2)
- **External:** `u32::try_from` — fractional-digit count
- **External:** `str::parse::< Backing >` (×2 — integer and fractional parts)
- **External:** `exact_scale::pow10` — scaling a short fractional part up to `SCALE` digits
- **External:** `i64::checked_mul`, `i64::checked_add` — magnitude accumulation
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:357`)
