# 002: impl Error for RoundError

## Representation

Marks `RoundError` as a standard error type (empty body).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_round/src/lib.rs:96`

```rust
impl core::error::Error for RoundError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 96 | Declaration — empty body |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Standard-error marker for `RoundError` |
