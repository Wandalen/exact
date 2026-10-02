# 002: Error for DustError

## Representation

Marks `DustError` as a standard error type, via the blanket-default
`core::error::Error` trait — no method body of its own.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_dust/src/lib.rs:88`

```rust
impl core::error::Error for DustError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 88 | Declaration |

No file anywhere calls a method on this impl — it declares none of its own.
An honest empty finding; the impl's entire purpose is the trait-bound marker
itself (`DustError: Error`), matching the identical pattern already recorded
for `exact_kind::KindError`, `exact_ratio::RatioError`,
`exact_snap::SnapError`, and `exact_fmt::FmtError`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Qualifies `DustError` as a standard error type |
