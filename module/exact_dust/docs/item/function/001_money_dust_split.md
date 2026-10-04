# 001: money_dust_split

## Representation

Split a money value into `parts` equal shares, rounding under `mode`,
sending the remainder to `to`. The allocating entry point — the crate's one
function with a confirmed real caller outside its own tests.

`DustError::Overflow` also covers a subtle case the module doc comment
discloses (`src/lib.rs:36-45`): the leftover is folded in at the raw
minor-unit level rather than through `Money`/`Quantity`'s own checked
arithmetic, because an `Up`/`HalfEven` rounding mode can make the collective
share allocation *exceed* the total — correcting slot 0 then means
subtracting, which `Quantity::checked_add` cannot express. Reconstructing
slot 0 via `from_minor` handles both directions uniformly and still refuses
a result that would go negative; see
`qty_dust_split_refuses_a_first_slot_that_would_go_negative_under_up_rounding`
in this crate's own tests for the concrete case.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:151-158`

```rust
pub fn money_dust_split( total : Money, parts : usize, mode : Rounding, to : DustTo ) -> Result< Vec< Money >, DustError >
{
  let ( share, leftover ) = split_minor( total.minor(), parts, mode )?;
  fill_minor( share, leftover, to, parts )?
  .into_iter()
  .map( | minor | Money::from_minor( minor ).map_err( | _ | DustError::Overflow ) )
  .collect()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 151-158 | Declaration |
| `tests/dust_split_test.rs` | 21-23,32,44,54,63,71,79,93 | Every split scenario this crate's own tests cover |
| `exact_arith/src/lib.rs:32` | — | Doctest call (crate-level `//! ``` ` example, compiled/run under `cargo test --doc`, not production) |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:38` | — | Test-only call exercising the facade re-export |
| `smoke_exact_market_split/src/lib.rs:134` | — | `market_split`'s own body — the only production (non-test, non-doctest) call site anywhere in the workspace |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised extensively by this crate's own tests |
| `exact_arith` | `src/lib.rs` | Re-export, plus the facade's own crate-doc example and test both actually call it — unlike every other function in this crate's sibling catalogs (`exact_add`/`exact_parse`/`exact_cmp`/`exact_snap`), where the facade's test bypasses the re-export entirely, `exact_arith` genuinely exercises this one |
| `smoke_exact_market_split` | `src/lib.rs` | **Production** — `market_split`'s one split operation, reached through the `exact_arith` facade re-export (confirmed via `smoke_exact_market_split/Cargo.toml`'s `exact_arith` path dependency — not a same-named collision) |

## Caller Tree

No caller within `exact_dust` itself, and no *production* caller within any
other crate's own source either, when scoped strictly to
non-test/non-doctest code — `smoke_exact_market_split::market_split`
(`smoke_exact_market_split/src/lib.rs:134`) is the one exception: a genuine
production call from a different crate.

- **External:** `smoke_exact_market_split::market_split` (`smoke_exact_market_split/src/lib.rs:134`)

Also reached, but out of scope for this tree per `item_des.rulebook.md` §
Instance Documentation : Completeness Verification ("test-only call sites
are out of scope for both trees"): `exact_arith`'s own crate-doc doctest
(`src/lib.rs:29`) and its `tests/facade_test.rs:38`.

## Callee Tree

- `split_minor` (`src/lib.rs:153`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:112`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- `fill_minor` (`src/lib.rs:154`, private — no Item Instance of its own)
- **External:** `exact_kind::Money::minor` (`src/lib.rs:153`), `exact_kind::Money::from_minor` (`src/lib.rs:156`)
