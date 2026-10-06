# 001: ConservationError

## Representation

Why a conservation check could not be completed, or found a discrepancy.
Renamed from `exact_audit::AuditError` per the preferred design, with its
shape changed too: `AuditError::AccumulatorOverflow { at_entry }` becomes the
field-less `Overflow` — the position-tracking `at_entry` is dropped rather
than preserved, since the doc specifies this crate's error shape explicitly
(module doc comment, `src/lib.rs:50-57`). `NotZero { got : i128 }` is new,
added for the typed `*_sum_assert_zero` functions; `i128` is chosen because a
non-negative `Quantity` has no typed representation for a negative `got`
(`src/lib.rs:58-64`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_conserve/src/lib.rs:106-118`

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ConservationError
{
  NotZero
  {
    got : i128,
  },
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 108-118 | Declaration |
| `src/lib.rs` | 136, 212, 258, 286 | Constructed on overflow/not-zero across `verify` and the 4 typed functions |
| `tests/conservation_test.rs` | throughout | Every error-path test |
| `cluster_economy/src/error.rs:19,79-81` | — | **Production** — wrapped into `MarketError::Audit` via a `From` impl |
| `cluster_economy/tests/economy_test.rs:493,549,559` | — | Constructed directly and downcast-matched |
| `exact_arith/tests/facade_test.rs:61` | — | Asserts `Overflow`'s rendered message |
| `exact_arith/src/lib.rs:141` | — | Facade re-export |

**Real production consumer outside this crate's own tier**:
`cluster_economy` wraps this error type into its own domain error enum — the
only error type in this entire migration confirmed to cross into a
downstream exchange/market crate's own error handling, not just its tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Every fallible function's error type |
| `exact_arith` | `src/lib.rs` | Re-export; own test asserts `Display` output |
| `cluster_economy` | `src/error.rs` | **Production** — wrapped via `From< ConservationError > for MarketError` |
