# 002: impl Error for KindError

## Representation

Marks `KindError` as a standard error type (empty body — `Display` already
supplies the message `core::error::Error` requires). Lets every downstream
crate's own error type participate in `?`-based error composition and
`Box<dyn Error>` contexts.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:138`

```rust
impl core::error::Error for KindError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 138 | Declaration — empty body, no further call sites of its own |

No file calls this impl's (absent, default-provided) methods directly; its
only effect is making `KindError: Error` hold, which downstream crates rely
on structurally (e.g. for `?`-composability) rather than through any call
site grep could find.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Standard-error marker for `KindError` |
