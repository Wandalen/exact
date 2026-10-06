# 002: Error for SnapError

## Representation

Marks `SnapError` as a standard error type, via the blanket-default
`core::error::Error` trait — no method body, since the default `source()`
(`None`) and the inherited `Display` already cover this crate's needs.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_snap/src/lib.rs:52`

```rust
impl core::error::Error for SnapError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 52 | Declaration |

No file anywhere calls a method on this impl — it declares none of its own,
taking every `Error` method at its default. An honest empty finding; the
impl's entire purpose is the trait-bound marker itself (`SnapError: Error`).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | Qualifies `SnapError` as a standard error type |
