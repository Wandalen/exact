# 001: FmtError

## Representation

Why a buffer-writing render could not complete. A single-variant enum today
— the only way `fmt_into` can fail is running out of buffer space.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_fmt/src/lib.rs:43-49`

```rust
/// Why a buffer-writing render could not complete.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum FmtError
{
  /// The buffer was too small to hold the rendered text.
  BufFull,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 45-49 | Declaration |
| `src/lib.rs` | 57 | Constructed in `Display for FmtError`'s one match arm |
| `src/lib.rs` | 99 | Constructed in `fmt_into` on a `write!` failure |
| `tests/fmt_test.rs:34` | — | Asserts `fmt_into` returns exactly `Err( FmtError::BufFull )` on a too-small buffer |
| `exact_arith/src/lib.rs:118` | — | Facade re-export |

No production call site anywhere constructs or matches `FmtError` outside
`exact_fmt` itself — an honest empty finding. `exact_arith` only re-exports
the name.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | The sole error type for `fmt_into`; exercised by its own buffer-overflow test |
| `exact_arith` | `src/lib.rs` | Re-export only |
