# 001: WireError

## Representation

Why a `Wire` could not be decoded, or a decoded value could not be turned
into a specific kind. Five variants: two structural (`BadKind`, `BadScale`),
one length-related (`Truncated`), and two range-related (`Overflow`,
`Negative`) — the latter two beyond the preferred design's own three-variant
listing, added because a decoded `minor` is a bare `i64` with no range check
of its own until a kind's own constructor sees it (module doc comment,
`src/lib.rs:15-24`), the same reason `exact_ratio::RatioError` needed the
identical two additions.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_bytes/src/lib.rs:58-75`

```rust
/// Why a `Wire` could not be decoded, or a decoded value could not be
/// turned into a specific kind.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum WireError
{
  /// The `kind` byte did not match the kind being decoded into.
  BadKind,
  /// The `scale` byte did not match the scale the kind expects.
  BadScale,
  /// The byte slice was shorter than [`Wire::ENCODED_LEN`].
  Truncated,
  /// The decoded `minor` value left the representable or declared range.
  Overflow,
  /// The decoded `minor` value was below zero, for a kind that refuses it.
  Negative
  {
    /// The offending count of minor units.
    minor : i64,
  },
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 59-75 | Declaration |
| `src/lib.rs` | 83-87 | Matched exhaustively in `Display for WireError` |
| `src/lib.rs` | 98-100 | Constructed in `kind_error_to_wire_error` |
| `src/lib.rs` | 111,115,190 | Constructed directly as the `Err` arm of `check_header`'s kind and scale guards (shared by every `*_from_wire`) and of `Wire::from_bytes`'s length guard |
| `tests/wire_roundtrip_test.rs` | 45,48,59,68,89,97 | Asserting the exact variant returned by each failure mode |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:6,72` | — | `BadScale` when money at scale 2 is read at scale 6 |
| `smoke_exact_market_split/src/lib.rs:51,410` | — | Step 7: `BadScale` when a satoshi quantity is read at scale 6 |

No production call site anywhere renders a `WireError` through its `Display`
impl — every real use is construction or `assert_eq!`/`matches!` pattern
matching. See [Display::fmt for WireError](../associated_function/001_fmt_display_for_wire_error.md).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The sole error type for every fallible function in the crate |
| `exact_arith` | `src/lib.rs`, `tests/any_scale_test.rs` | Re-export; `BadScale` asserted at scale 2 |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: `BadScale` asserted in step 7 |
