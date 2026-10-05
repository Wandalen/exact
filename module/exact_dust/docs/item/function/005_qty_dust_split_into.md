# 005: qty_dust_split_into

## Representation

Non-allocating variant of [qty_dust_split](004_qty_dust_split.md) — writes
into `out` instead of returning a `Vec`; `out.len()` is the part count.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:221-230`

```rust
pub fn qty_dust_split_into( total : Quantity, mode : Rounding, to : DustTo, out : &mut [ Quantity ] ) -> Result< (), DustError >
{
  let ( share, leftover ) = split_minor( total.minor(), out.len(), mode )?;
  for ( i, slot ) in out.iter_mut().enumerate()
  {
    let minor = slot_minor( share, leftover, to, i )?;
    *slot = Quantity::from_minor( minor ).map_err( | _ | DustError::Overflow )?;
  }
  Ok( () )
}
```

The same per-slot shape as
[money_dust_split_into](002_money_dust_split_into.md): no intermediate `Vec`,
so no heap allocation, and a refusal on slot 0 leaves `out` untouched.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 221-230 | Declaration |
| `tests/dust_split_test.rs:94` | — | Writes the same shares as the allocating `qty_dust_split` — under `Up`, where slot 0 absorbs a negative leftover |
| `tests/dust_split_test.rs:167` | — | An empty buffer is refused as `EmptyParts` |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |

No call site outside this crate's own tests — not `exact_arith`'s crate-doc
example or test suite, not `smoke_exact_market_split`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by its own tests — parity with `qty_dust_split`, and the empty buffer |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller outside this crate's own tests.

## Callee Tree

- `split_minor` (`src/lib.rs:223`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:125`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- `slot_minor` (`src/lib.rs:226`, private — no Item Instance of its own), once per slot
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:223`), `exact_kind::Quantity::from_minor` (`src/lib.rs:227`)
