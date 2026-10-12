# 005: qty_dust_split_into

## Representation

Non-allocating variant of [qty_dust_split](004_qty_dust_split.md) — writes
into `out` instead of returning a `Vec`; `out.len()` is the part count.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_dust/src/lib.rs:256-259`

```rust
pub fn qty_dust_split_into< const S : u32 >( total : Qty< S >, mode : Rounding, to : DustTo, out : &mut [ Qty< S > ] ) -> Result< (), DustError >
{
  split_into_with( total.minor(), mode, to, out, Qty::< S >::from_minor )
}
```

The body is the private `split_into_with` (`src/lib.rs:173-200`), shared
with [money_dust_split_into](002_money_dust_split_into.md); the `Fix(exact_dust_split_into_allocated)` comment lives
there, above the per-slot loop.

The same per-slot shape as
[money_dust_split_into](002_money_dust_split_into.md): no intermediate `Vec`,
so no heap allocation, and a refusal on slot 0 leaves `out` untouched.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 256-259 | Declaration |
| `tests/dust_split_test.rs:94` | — | Writes the same shares as the allocating `qty_dust_split` — under `Up`, where slot 0 absorbs a negative leftover |
| `tests/dust_split_test.rs:167` | — | An empty buffer is refused as `EmptyParts` |
| `exact_arith/src/lib.rs:140` | — | Facade re-export |
| `exact_arith/tests/no_alloc_test.rs:93` | — | Test-only call checking the split makes no heap allocation |

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

- `split_into_with` (`src/lib.rs:258`, private — no Item Instance of its own)
  - `split_minor` (`src/lib.rs:185`, private — no Item Instance of its own)
    - `round_error_to_dust_error` (`src/lib.rs:132`, private — no Item Instance of its own, invoked via `.map_err(...)` on `round_div`'s result)
    - **External:** `exact_round::round_div`
  - `slot_minor` (`src/lib.rs:196`, private — no Item Instance of its own), once per slot
- **External:** `exact_kind::Quantity::minor` (`src/lib.rs:258`), `exact_kind::Quantity::from_minor` (`src/lib.rs:258`, passed to `split_into_with` as `make`, called at `190`)
