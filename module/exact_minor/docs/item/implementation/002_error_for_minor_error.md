# 002: impl Error for MinorError

## Representation

Marks `MinorError` as a standard error type (empty body — `Display` already
supplies the message).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_minor/src/lib.rs:51`

```rust
impl core::error::Error for MinorError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 51 | Declaration — empty body |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Standard-error marker for `MinorError` |
