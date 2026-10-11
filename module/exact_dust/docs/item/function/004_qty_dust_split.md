# 004: qty_dust_split

## Representation

Split a quantity into `parts` equal shares, rounding under `mode`, sending
the remainder to `to`. As [money_dust_split](001_money_dust_split.md), but
over the non-negative kind — this is where an `Up`/`HalfEven` rounding mode
can actually drive the first-slot correction negative and trigger
`DustError::Overflow`, since `Quantity` (unlike `Money`) refuses to hold a
negative value at all.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:246-249`

```rust
pub fn qty_dust_split< const S : u32 >( total : Qty< S >, parts : usize, mode : Rounding, to : DustTo ) -> Result< Vec< Qty< S > >, DustError >
{
  split_with( total.minor(), parts, mode, to, Qty::< S >::from_minor )
}
```

The body is the private `split_with` (`src/lib.rs:155-171`), shared with
[money_dust_split](001_money_dust_split.md): this function only supplies the
total's minor count and `Quantity::from_minor` to build each slot.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 246-249 | Declaration |
| `tests/dust_split_test.rs:97,136,152,230` | — | Parity with `qty_dust_split_into`, a clean `Down`-rounded split, the `Up`-rounded case that refuses a first slot that would go negative, and `Exact` refusing an uneven split |
| `exact_arith/src/lib.rs:139` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:50` | — | Splits a quantity in satoshis |
| `smoke_exact_market_split/src/lib.rs:413` | — | Step 7 splits a satoshi quantity among three accounts |

Outside this crate's own tests, it is called only at other scales: at
scale 8 by `exact_arith`'s any-scale test and by the demo lane's step 7.
`exact_arith`'s crate-doc example exercises only the `Money` side.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by 2 of this crate's own tests, including the crate's one negative-correction edge case |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export; called at scale 8 by the any-scale facade test |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: step 7's split at scale 8 |

## Caller Tree

- **External:** `smoke_exact_market_split::per_asset_trade` (`smoke_exact_market_split/src/lib.rs:413`) — demo-lane code, not production

No production caller.

## Callee Tree

- `split_with` (`src/lib.rs:248`, private — no Item Instance of its own)
  - `split_minor` (`src/lib.rs:167`, private — no Item Instance of its own)
    - `round_error_to_dust_error` (`src/lib.rs:132`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
    - **External:** `exact_round::round_div`
  - `slot_minor` (`src/lib.rs:169`, private — no Item Instance of its own), once per slot
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:248`), `exact_kind::Quantity::from_minor` (`src/lib.rs:248`, passed to `split_with` as `make`, called at `162`)
