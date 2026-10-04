# 001: MinorError

## Representation

Why a checked operation could not be completed — `Overflow` when the result
would be above the backing width, `Underflow` when it would be below it.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_minor/src/lib.rs:188`

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
| `src/lib.rs` | 88,90,95-96,124,129-130,141,146-147,157,162,188,204,216,238,243-244,254,259-260,270,275 | Return type / constructed variant of all 3 checked functions, and both trait impls it carries |
| `tests/checked_arithmetic_test.rs` | throughout | Matched by equality against the exact variant and `operation` string |
| `exact_arith/src/lib.rs:69` | — | Facade re-export only |

`exact_kind` receives one from the 3 checked functions and matches it in its
private `kind_overflow` (`exact_kind/src/lib.rs:164-171`), turning both variants into
`KindError::Overflow`; nothing else outside this crate does.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Every checked operation's error type; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |
