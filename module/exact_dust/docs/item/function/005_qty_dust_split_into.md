# 005: qty_dust_split_into

## Representation

Non-allocating variant of [qty_dust_split](004_qty_dust_split.md). The
sharpest finding in this crate: zero references anywhere in the workspace
beyond its own declaration and the facade's re-export — not even this
crate's own test suite exercises it, unlike every one of its 5 sibling
functions.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:210-219`

```rust
pub fn qty_dust_split_into( total : Quantity, mode : Rounding, to : DustTo, out : &mut [ Quantity ] ) -> Result< (), DustError >
{
  let ( share, leftover ) = split_minor( total.minor(), out.len(), mode )?;
  let minors = fill_minor( share, leftover, to, out.len() )?;
  for ( slot, minor ) in out.iter_mut().zip( minors )
  {
    *slot = Quantity::from_minor( minor ).map_err( | _ | DustError::Overflow )?;
  }
  Ok( () )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 210-219 | Declaration |
| `exact_arith/src/lib.rs:121` | — | Facade re-export |

Confirmed via a dedicated workspace-wide search
(`grep -rn qty_dust_split_into --include='*.rs'`) that these are the only 2
occurrences anywhere — not this crate's own `tests/dust_split_test.rs` (which
covers `money_dust_split_into` but never the `Quantity` counterpart), not
`exact_arith`'s crate-doc example or test suite, not
`smoke_exact_market_split`. The one function in this entire crate untested
by its own defining crate.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, and the
only function in this crate untested even by its own defining crate.

## Callee Tree

- `split_minor` (`src/lib.rs:212`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:112`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- `fill_minor` (`src/lib.rs:213`, private — no Item Instance of its own)
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:212`), `exact_kind::Quantity::from_minor` (`src/lib.rs:216`)
