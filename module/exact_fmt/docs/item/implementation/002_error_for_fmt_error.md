# 002: Error for FmtError

## Representation

Marks `FmtError` as a standard error type, via the blanket-default
`core::error::Error` trait — no method body, since the default `source()`
(`None`) and the inherited `Display` already cover this crate's needs.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_fmt/src/lib.rs:62`

```rust
impl core::error::Error for FmtError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 62 | Declaration |

No file anywhere calls a method on this impl — it declares none of its own,
taking every `Error` method at its default. An honest empty finding; the
impl's entire purpose is the trait-bound marker itself (`FmtError: Error`),
not any method it provides.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Qualifies `FmtError` as a standard error type |
