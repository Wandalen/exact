# 002: DustError

## Representation

Why a split could not be computed. `Overflow` also absorbs a case the
preferred design doesn't name separately: `exact_snap` already set the
precedent of folding a reachable-but-rare condition into `Overflow` rather
than adding a dedicated variant, and this crate matches it — see
[money_dust_split](../function/001_money_dust_split.md)'s Representation for
the specific negative-leftover scenario this covers. `Remainder` is reported
both for `DustTo::Reject` and for `Rounding::Exact`, whose
`exact_round::RoundError::Inexact` is the same fact — the split did not
divide evenly.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_dust/src/lib.rs:76-87`

```rust
/// Why a split could not be computed.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DustError
{
  /// Zero parts were requested.
  EmptyParts,
  /// The split did not divide evenly and [`DustTo::Reject`] or
  /// [`Rounding::Exact`] was asked.
  Remainder,
  /// An operation left the representable or declared range.
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 78-87 | Declaration |
| `src/lib.rs` | 89-100 | `Display` match arms |
| `src/lib.rs` | 113-114,117,126,128,130-131,143,147,166,194 | Constructed across `round_error_to_dust_error` (`Inexact` → `Remainder` at 117), `split_minor`, `slot_minor`, and the `.map_err` closures of `split_with` and `split_into_with` (all private — no Item Instance of their own) |
| `tests/dust_split_test.rs:58,75,152` | — | Asserts the exact variant returned for `Remainder`, `EmptyParts`, and `Overflow` respectively |
| `exact_arith/src/lib.rs:139` | — | Facade re-export |

No production call site anywhere constructs or matches `DustError` outside
`exact_dust` itself — an honest empty finding; `exact_arith` only re-exports
the name, and `smoke_exact_market_split`'s own call site discards any error
via `.expect(...)` rather than matching the enum.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | The sole error type for every public function; exercised by 3 of this crate's own tests covering all 3 variants |
| `exact_arith` | `src/lib.rs` | Re-export only |
