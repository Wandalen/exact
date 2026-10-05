# 006: qty_dust_remainder

## Representation

The remainder a [qty_dust_split](004_qty_dust_split.md) would hold back. As
[money_dust_remainder](003_money_dust_remainder.md), never calls
`fill_minor` — only the leftover is needed.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:237-241`

```rust
pub fn qty_dust_remainder( total : Quantity, parts : usize, mode : Rounding ) -> Result< i64, DustError >
{
  let ( _share, leftover ) = split_minor( total.minor(), parts, mode )?;
  Ok( leftover )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 237-241 | Declaration |
| `tests/dust_split_test.rs:162` | — | Confirms the held-back amount matches the `Money` case's figure (identical minor-unit arithmetic) |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |

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

- `split_minor` (`src/lib.rs:239`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:125`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:239`)

No call to `fill_minor`, the same structural omission as
`money_dust_remainder`'s Callee Tree.
