# 007: money_dust_split_weighted

## Representation

Split a money value in proportion to whole-number weights — slot `i` gets
`total × weights[ i ] / Σ weights`, rounded under `mode` — and send the
remainder to `to`, exactly as [money_dust_split](001_money_dust_split.md)
does. Built for fees split by percentage (E4, for the exchange's X8): `$10.01`
at `[ 70, 20, 10 ]` under `Down`/`First` is `[ 7.01, 2.00, 1.00 ]`. A
separately named operation, as [ADR-002](../../decisions/002_equal_count_split_surface.md)
required (→ [ADR-004](../../decisions/004_weighted_split_as_a_named_operation.md)).

Only `TowardZero` keeps every part on the total's side of zero. Any other
mode can round the shares past the total, and `First` then takes the excess
from slot 0, even at weight 0: `0.01` at `[ 0, 1, 1 ]` under `Up` is
`[ -0.01, 0.01, 0.01 ]`, and `Down` does the same to a negative total.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:308-332`

```rust
pub fn money_dust_split_weighted< const S : u32 >( total : Decimal< S >, weights : &[ u32 ], mode : Rounding, to : DustTo ) -> Result< Vec< Decimal< S > >, DustError >
{
  let total_minor = total.minor();
  let sum = weights.iter().try_fold( 0_u64, | acc, &w | acc.checked_add( u64::from( w ) ) ).ok_or( DustError::Overflow )?;
  if sum == 0
  {
    return Err( DustError::EmptyParts );
  }
  // Each share once, straight into the returned `Vec`, summed on the way.
  let mut parts = Vec::with_capacity( weights.len() );
  let mut allocated : i64 = 0;
  for &w in weights
  {
    let share = weighted_share( total_minor, w, sum, mode )?;
    allocated = allocated.checked_add( share ).ok_or( DustError::Overflow )?;
    parts.push( Decimal::< S >::from_minor( share ).map_err( | _ | DustError::Overflow )? );
  }
  // The leftover goes where `to` says; only slot 0 can change.
  let leftover = total_minor.checked_sub( allocated ).ok_or( DustError::Overflow )?;
  let first = slot_minor( parts[ 0 ].minor(), leftover, to, 0 )?;
  parts[ 0 ] = Decimal::< S >::from_minor( first ).map_err( | _ | DustError::Overflow )?;
  Ok( parts )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 308-332 | Declaration |
| `src/lib.rs` | 292-299 | Doc example: `$10.00` at `[ 1, 1, 1 ]` is `[ 3.34, 3.33, 3.33 ]` |
| `tests/weighted_split_test.rs` | throughout | Every `DustTo`, `Exact`, `Reject` on a remainder and not a rounded share, empty and zero weights, signs under `TowardZero`, equal weights against `money_dust_split`, conservation over 70 035 cases under `First` and `Sink`, no overflow at `Money::MAX` |
| `exact_arith/src/lib.rs:140` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:75-83` | — | A fee in cents split 70/20/10 through the facade |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by its own test file and doc example |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export, and one call through the facade at scale 2 |

## Caller Tree

No caller anywhere, intra-crate or external, in production code — an honest
empty tree. The exchange's fees (X8) are the intended caller, once a fee
policy is decided.

## Callee Tree

- `weighted_share` (`src/lib.rs:275`, private — no Item Instance of its own), once per slot: each share is summed and built in one pass
  - **External:** `exact_round::round_div_wide`, on `total × weight` widened to `i128`
  - `round_error_to_dust_error` (`src/lib.rs:107`, private — no Item Instance of its own), via `.map_err(...)`
- `slot_minor` (`src/lib.rs:142`, private — no Item Instance of its own), once, for slot 0 — the same leftover distribution the equal split uses
- **External:** `exact_kind::Decimal::minor` (`src/lib.rs:310`), `exact_kind::Decimal::from_minor` (`src/lib.rs:329`)
