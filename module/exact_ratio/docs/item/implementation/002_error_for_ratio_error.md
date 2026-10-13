# 002: impl Error for RatioError

## Representation

Marks `RatioError` as a standard error type (empty body — `Display` already
supplies the message `core::error::Error` requires). Lets `RatioError`
participate in `?`-based error composition and `Box<dyn Error>` contexts.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_ratio/src/lib.rs:82`

```rust
impl core::error::Error for RatioError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 82 | Declaration — empty body, no further call sites of its own |

No file calls this impl's (absent, default-provided) methods directly; its
only effect is making `RatioError: Error` hold.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Standard-error marker for `RatioError` |
