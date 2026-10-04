# 002: money_dust_split_into

## Representation

Non-allocating variant of [money_dust_split](001_money_dust_split.md) —
writes into `out` instead of returning a `Vec`; `out.len()` is the part
count.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:166-175`

```rust
pub fn money_dust_split_into( total : Money, mode : Rounding, to : DustTo, out : &mut [ Money ] ) -> Result< (), DustError >
{
  let ( share, leftover ) = split_minor( total.minor(), out.len(), mode )?;
  let minors = fill_minor( share, leftover, to, out.len() )?;
  for ( slot, minor ) in out.iter_mut().zip( minors )
  {
    *slot = Money::from_minor( minor ).map_err( | _ | DustError::Overflow )?;
  }
  Ok( () )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 166-175 | Declaration |
| `tests/dust_split_test.rs:81` | — | Confirms it writes the same shares as the allocating `money_dust_split` |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |

No call site anywhere outside this crate's own single test — an honest
empty finding. `exact_arith` only re-exports the name; neither its crate-doc
example nor its test suite calls this variant (both exercise
`money_dust_split` instead).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by its one parity test against `money_dust_split` |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- `split_minor` (`src/lib.rs:168`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:112`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- `fill_minor` (`src/lib.rs:169`, private — no Item Instance of its own)
- **External:** `exact_kind::Money::minor` (`src/lib.rs:168`), `exact_kind::Money::from_minor` (`src/lib.rs:172`)
