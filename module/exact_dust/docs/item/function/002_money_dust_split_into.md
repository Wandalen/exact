# 002: money_dust_split_into

## Representation

Non-allocating variant of [money_dust_split](001_money_dust_split.md) —
writes into `out` instead of returning a `Vec`; `out.len()` is the part
count.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:177-194`

```rust
pub fn money_dust_split_into( total : Money, mode : Rounding, to : DustTo, out : &mut [ Money ] ) -> Result< (), DustError >
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
    *slot = Money::from_minor( minor ).map_err( | _ | DustError::Overflow )?;
  }
  Ok( () )
}
```

Each slot's value is computed by `slot_minor` and written straight into
`out` — no intermediate `Vec`, so the call makes no heap allocation. Under
[`DustTo::Reject`] with a remainder, `slot_minor` refuses on slot 0, before
anything is written.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 177-194 | Declaration |
| `tests/dust_split_test.rs:80` | — | Writes the same shares as the allocating `money_dust_split` |
| `tests/dust_split_test.rs:106` | — | `DustTo::Reject` refuses before writing — the buffer keeps what it held |
| `tests/dust_split_test.rs:167` | — | An empty buffer is refused as `EmptyParts` |
| `tests/dust_split_test.rs:181` | — | `DustTo::Sink` leaves every slot at the plain share |
| `exact_arith/src/lib.rs:139` | — | Facade re-export |
| `exact_arith/tests/no_alloc_test.rs:72` | — | Test-only call checking the split makes no heap allocation |

No production call site anywhere outside this crate. `exact_arith`'s
crate-doc example exercises `money_dust_split` instead; its
`tests/no_alloc_test.rs` calls this variant to count allocations.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Exercised by its own tests — parity with `money_dust_split`, refusal, empty and `Sink` buffers |
| `exact_arith` | `src/lib.rs`, `tests/no_alloc_test.rs` | Re-export; its allocation test calls it |

## Caller Tree

- **External:** `exact_arith`'s own test, `no_alloc_test.rs:72` (test-context, via the re-exported name)

## Callee Tree

- `split_minor` (`src/lib.rs:179`, private — no Item Instance of its own)
  - `round_error_to_dust_error` (`src/lib.rs:125`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
  - **External:** `exact_round::round_div`
- `slot_minor` (`src/lib.rs:190`, private — no Item Instance of its own), once per slot
- **External:** `exact_kind::Money::minor` (`src/lib.rs:179`), `exact_kind::Money::from_minor` (`src/lib.rs:191`)
