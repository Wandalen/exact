# 006: impl Display for Qty

## Representation

Delegates entirely to the wrapped `Decimal`'s own `Display` — a non-negative
quantity renders identically to the signed decimal it holds (which is always
`>= 0`). It passes the formatter itself through, not `"{}"`, so a requested
precision (`{:.2}`) reaches the decimal.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:696`

```rust
impl< const SCALE : u32 > fmt::Display for Qty< SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    // Forward the formatter itself, not `"{}"`, so a precision reaches the decimal.
    fmt::Display::fmt( &self.value, f )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 696-703 | Declaration |
| `tests/non_negative_test.rs:50` | — | `qty.to_string()` compared against the inner decimal's own rendering |
| `tests/display_precision_test.rs` | — | A precision passed through to the decimal |
| `exact_fmt/src/lib.rs:114` | — | `qty_fmt`'s `v.to_string()` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own test suite |
| `exact_fmt` | `src/lib.rs` | `qty_fmt`'s entire implementation is this `Display` via `.to_string()` |
