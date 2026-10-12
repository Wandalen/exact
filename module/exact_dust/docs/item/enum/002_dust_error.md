# 002: DustError

## Representation

Why a split could not be computed. `Overflow` also absorbs a case the
preferred design doesn't name separately: `exact_snap` already set the
precedent of folding a reachable-but-rare condition into `Overflow` rather
than adding a dedicated variant, and this crate matches it — see
[money_dust_split](../function/001_money_dust_split.md)'s Representation for
the specific negative-leftover scenario this covers. `Remainder` is reported
both for `DustTo::Reject` — the rounded parts leave a remainder — and for
`Rounding::Exact`, whose `exact_round::RoundError::Inexact` means a share did
not divide evenly. For the equal split the two are the same fact; a weighted
split can round one share up and another down and leave no remainder, which
`Reject` then accepts.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_dust/src/lib.rs:79-90`

```rust
/// Why a split could not be computed.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DustError
{
  /// Zero parts were requested — a part count of zero, or no nonzero weight.
  EmptyParts,
  /// The rounded parts left a remainder that [`DustTo::Reject`] refuses, or
  /// a share did not divide evenly under [`Rounding::Exact`].
  Remainder,
  /// An operation left the representable or declared range.
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 81-90 | Declaration |
| `src/lib.rs` | 92-103 | `Display` match arms |
| `src/lib.rs` | 116-117,120,129,131,133-134,146,150,169,197,279,320,323,331-332,335,337 | Constructed across `round_error_to_dust_error` (`Inexact` → `Remainder` at 117), `split_minor`, `slot_minor`, and the `.map_err` closures of `split_with` and `split_into_with` (all private — no Item Instance of their own); and in `weighted_share` and `money_dust_split_weighted` — `EmptyParts` for no weight to split by (323), `Overflow` on a checked sum or the ceiling |
| `tests/dust_split_test.rs:58,75,152` | — | Asserts the exact variant returned for `Remainder`, `EmptyParts`, and `Overflow` respectively |
| `tests/dust_split_test.rs:223,226,230,238,240` | — | `Remainder` under `Exact`, for an uneven split, buffer and remainder |
| `exact_arith/src/lib.rs:140` | — | Facade re-export |

No production call site anywhere constructs or matches `DustError` outside
`exact_dust` itself — an honest empty finding; `exact_arith` only re-exports
the name, and `smoke_exact_market_split`'s own call site discards any error
via `.expect(...)` rather than matching the enum.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | The sole error type for every public function; exercised by 3 of this crate's own tests covering all 3 variants |
| `exact_arith` | `src/lib.rs` | Re-export only |
