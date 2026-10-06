# 001: MinorError

## Representation

Why a checked operation could not be completed — `Overflow` when the result
would be above the backing width, `Underflow` when it would be below it.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_minor/src/lib.rs:206`

```rust
pub enum MinorError
{
  /// The result would have been above the largest value the backing width holds.
  Overflow
  {
    /// Which operation — `add`, `sub`, `neg`.
    operation : &'static str,
  },
  /// The result would have been below the smallest value the backing width holds.
  Underflow
  {
    /// Which operation — `add`, `sub`.
    operation : &'static str,
  },
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 106,108,113-114,142,147-148,159,164-165,175,180,206,222,234,256,261-262,272,277-278,288,293 | Return type / constructed variant of all 6 checked functions (3 `minor_*`, 3 `minor_wide_*` behind the `i128` feature) and of `TryFrom< MinorWide > for Minor`, and both trait impls it carries |
| `tests/checked_arithmetic_test.rs` | throughout | Matched by equality against the exact variant and `operation` string |
| `tests/wide_test.rs` | 7,43-62,88-107,109-113 | The same, for the `i128` narrowing and `minor_wide_*` functions |
| `exact_arith/src/lib.rs:70` | — | Facade re-export only |

`exact_kind` receives one from the 3 checked functions and matches it in its
private `kind_overflow` (`exact_kind/src/lib.rs:164-171`), turning both variants into
`KindError::Overflow`; nothing else outside this crate does.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Every checked operation's error type; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |
