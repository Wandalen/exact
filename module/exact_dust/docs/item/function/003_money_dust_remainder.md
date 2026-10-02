# 003: money_dust_remainder

## Representation

The remainder a [money_dust_split](001_money_dust_split.md) of `total` into
`parts` under `mode` would hold back, independent of where a `DustTo` would
send it. Unlike the split functions, this one never calls `fill_minor` — it
only needs the leftover, not a built set of output slots.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:184-188`

```rust
pub fn money_dust_remainder( total : Money, parts : usize, mode : Rounding ) -> Result< i64, DustError >
{
  let ( _share, leftover ) = split_minor( total.minor(), parts, mode )?;
  Ok( leftover )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 184-188 | Declaration |
| `tests/dust_split_test.rs:46` | — | Confirms the held-back amount under `DustTo::Sink` |
| `exact_arith/src/lib.rs:121` | — | Facade re-export |

No call site anywhere outside this crate's own single test — an honest
empty finding. `exact_arith` only re-exports the name.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by its one remainder-reporting test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- `split_minor` (`src/lib.rs:186`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:112`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- **External:** `exact_kind::Money::minor` (`src/lib.rs:186`)

No call to `fill_minor` — this function returns the raw leftover and never
builds a set of output slots, the one structural difference from
`money_dust_split`/`money_dust_split_into`'s Callee Tree.
