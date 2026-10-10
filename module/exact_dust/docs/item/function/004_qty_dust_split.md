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

`module/exact_dust/src/lib.rs:243-246`

```rust
pub fn qty_dust_split( total : Quantity, parts : usize, mode : Rounding, to : DustTo ) -> Result< Vec< Quantity >, DustError >
{
  split_with( total.minor(), parts, mode, to, Quantity::from_minor )
}
```

The body is the private `split_with` (`src/lib.rs:152-168`), shared with
[money_dust_split](001_money_dust_split.md): this function only supplies the
total's minor count and `Quantity::from_minor` to build each slot.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 243-246 | Declaration |
| `tests/dust_split_test.rs:97,136,152,230` | — | Parity with `qty_dust_split_into`, a clean `Down`-rounded split, the `Up`-rounded case that refuses a first slot that would go negative, and `Exact` refusing an uneven split |
| `exact_arith/src/lib.rs:139` | — | Facade re-export |

No call site anywhere outside this crate's own 3 tests — an honest empty
finding. `exact_arith` only re-exports the name; neither its crate-doc
example nor its test suite calls this function (both only exercise the
`Money` side).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by 2 of this crate's own tests, including the crate's one negative-correction edge case |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- `split_with` (`src/lib.rs:245`, private — no Item Instance of its own)
  - `split_minor` (`src/lib.rs:164`, private — no Item Instance of its own)
    - `round_error_to_dust_error` (`src/lib.rs:129`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
    - **External:** `exact_round::round_div`
  - `slot_minor` (`src/lib.rs:166`, private — no Item Instance of its own), once per slot
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:245`), `exact_kind::Quantity::from_minor` (`src/lib.rs:245`, passed to `split_with` as `make`, called at `162`)
