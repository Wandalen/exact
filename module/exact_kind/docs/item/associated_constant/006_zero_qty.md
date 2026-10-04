# 006: Qty::ZERO

## Representation

Nothing held — the one infallible constructor, wrapping `Decimal::ZERO`
(which is always non-negative, so the wrap never fails).

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:420`

```rust
pub const ZERO : Self = Self { value : Decimal::ZERO };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 420 | Declaration |
| `tests/non_negative_test.rs` | throughout | Accumulator seed, boundary comparisons |
| `exact_conserve/tests/conservation_test.rs:141,169,171` | — | `try_fold` seed; `qty_sum_assert_zero` inputs |
| `exact_snap/tests/snap_test.rs:12,13,73` | — | `Tick`/`Lot` zero-rejection and zero-result checks |
| `exact_arith/tests/facade_test.rs:33` | — | Remainder-check comparison |

No production (non-test) file outside `exact_kind` uses `Quantity::ZERO`
directly — every external reference found is test-context. Contrast
[Money::ZERO](002_zero_decimal.md), which `smoke_exact_market_split` uses in
production.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own tests |
| `exact_conserve`, `exact_snap`, `exact_arith` | `tests/*.rs` | Test-only seed/boundary input |
