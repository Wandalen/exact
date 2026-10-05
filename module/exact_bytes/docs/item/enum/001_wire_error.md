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

`module/exact_bytes/src/lib.rs:52-69`

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
| `src/lib.rs` | 53-69 | Declaration |
| `src/lib.rs` | 77-81 | Matched exhaustively in `Display for WireError` |
| `src/lib.rs` | 92-93 | Constructed in `kind_error_to_wire_error` |
| `src/lib.rs` | 167,192,196,220,224,245,249 | Constructed directly as the `Err` arm of each `*_from_wire`/`Wire::from_bytes` guard |
| `tests/wire_roundtrip_test.rs` | 44,47,58,67,88,96 | Asserting the exact variant returned by each failure mode |
| `exact_arith/src/lib.rs:114` | — | Facade re-export |

No production call site anywhere renders a `WireError` through its `Display`
impl — every real use is construction or `assert_eq!`/`matches!` pattern
matching. See [Display::fmt for WireError](../associated_function/001_fmt_display_for_wire_error.md).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `src/lib.rs` | The sole error type for every fallible function in the crate |
| `exact_arith` | `src/lib.rs` | Re-export only |
