# 005: qty_dust_split_into

## Representation

Non-allocating variant of [qty_dust_split](004_qty_dust_split.md) — writes
into `out` instead of returning a `Vec`; `out.len()` is the part count.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:229-246`

```rust
pub fn qty_dust_split_into( total : Quantity, mode : Rounding, to : DustTo, out : &mut [ Quantity ] ) -> Result< (), DustError >
{
  let ( share, leftover ) = split_minor( total.minor(), out.len(), mode )?;
  // Fix(exact_dust_split_into_allocated): every slot's count used to be
  // collected into a `Vec` by `fill_minor` and then copied into `out` — one
  // heap allocation per call, against type/008's "does not allocate". Each
  // slot is now computed in place by `slot_minor`.
  //
  // Root cause: the `_into` variant reused the allocating `_split` helper.
  // Pitfall: a helper shared by an allocating and a non-allocating variant
  //   gives both the allocation, and the output is the same either way.
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
| `src/lib.rs` | 229-246 | Declaration |
| `tests/dust_split_test.rs:94` | — | Writes the same shares as the allocating `qty_dust_split` — under `Up`, where slot 0 absorbs a negative leftover |
| `tests/dust_split_test.rs:167` | — | An empty buffer is refused as `EmptyParts` |
| `exact_arith/src/lib.rs:139` | — | Facade re-export |
| `exact_arith/tests/no_alloc_test.rs:73` | — | Test-only call checking the split makes no heap allocation |

No production call site outside this crate — not `exact_arith`'s crate-doc
example, not `smoke_exact_market_split`; only `exact_arith`'s
`tests/no_alloc_test.rs` calls it, to count allocations.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by its own tests — parity with `qty_dust_split`, and the empty buffer |
| `exact_arith` | `src/lib.rs`, `tests/no_alloc_test.rs` | Re-export; its allocation test calls it |

## Caller Tree

- **External:** `exact_arith`'s own test, `no_alloc_test.rs:73` (test-context, via the re-exported name)

## Callee Tree

- `split_minor` (`src/lib.rs:231`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:125`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- `slot_minor` (`src/lib.rs:242`, private — no Item Instance of its own), once per slot
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:231`), `exact_kind::Quantity::from_minor` (`src/lib.rs:243`)
