# 007: money_dust_split_weighted

## Representation

Split a money value in proportion to whole-number weights — slot `i` gets
`total × weights[ i ] / Σ weights`, rounded under `mode` — and send the
remainder to `to`, exactly as [money_dust_split](001_money_dust_split.md)
does. Built for fees split by percentage (E4, for the exchange's X8): `$10.01`
at `[ 70, 20, 10 ]` under `Down`/`First` is `[ 7.01, 2.00, 1.00 ]`. A
separately named operation, as [ADR-002](../../decisions/002_equal_count_split_surface.md)
required (→ [ADR-004](../../decisions/004_weighted_split_as_a_named_operation.md)).

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
  // First pass: what the rounded shares add up to, so the leftover is known
  // before any slot is built.
  let mut allocated : i64 = 0;
  for &w in weights
  {
    allocated = allocated.checked_add( weighted_share( total_minor, w, sum, mode )? ).ok_or( DustError::Overflow )?;
  }
  let leftover = total_minor.checked_sub( allocated ).ok_or( DustError::Overflow )?;
  // Second pass: each slot's share, the leftover sent where `to` says.
  weights.iter().enumerate()
  .map( | ( i, &w ) |
  {
    let minor = slot_minor( weighted_share( total_minor, w, sum, mode )?, leftover, to, i )?;
    Decimal::< S >::from_minor( minor ).map_err( | _ | DustError::Overflow )
  } )
  .collect()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 308-332 | Declaration |
| `src/lib.rs` | 292-299 | Doc example: `$10.00` at `[ 1, 1, 1 ]` is `[ 3.34, 3.33, 3.33 ]` |
| `tests/weighted_split_test.rs` | throughout | Every `DustTo`, `Exact`, empty and zero weights, equal weights against `money_dust_split`, conservation over about 100 000 cases, no overflow at `Money::MAX` |
| `exact_arith/src/lib.rs:139` | — | Facade re-export |
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

- `weighted_share` (`src/lib.rs:275`, private — no Item Instance of its own), twice per slot: once to sum the shares, once to build them
  - **External:** `exact_round::round_div_wide`, on `total × weight` widened to `i128`
  - `round_error_to_dust_error` (`src/lib.rs:107`, private — no Item Instance of its own), via `.map_err(...)`
- `slot_minor` (`src/lib.rs:142`, private — no Item Instance of its own), once per slot — the same leftover distribution the equal split uses
- **External:** `exact_kind::Decimal::minor` (`src/lib.rs:310`), `exact_kind::Decimal::from_minor` (`src/lib.rs:329`)
