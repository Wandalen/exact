# 002: Error for WireError

## Representation

Marks `WireError` as a standard error type via the blanket-default
`core::error::Error` trait — no method body, taking the default `source()`
(`None`).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_bytes/src/lib.rs:90`

```rust
impl core::error::Error for WireError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 90 | Declaration |

No file anywhere calls a method on this impl — it declares none of its own.
An honest empty finding; the impl's entire purpose is the trait-bound marker
itself (`WireError: Error`).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Qualifies `WireError` as a standard error type |
