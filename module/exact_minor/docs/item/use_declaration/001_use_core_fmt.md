# 001: use core::fmt

## Representation

Brings the `core::fmt` module into scope under its own name, so
`MinorError`'s `Display` impl can write `fmt::Display`/`fmt::Formatter`/
`fmt::Result` instead of the fully qualified path. Private — not re-exported
— so it has no existence outside this file.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_minor/src/lib.rs:33`

```rust
use core::fmt;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 204,206 | `MinorError`'s `Display` impl header and `fmt`'s own signature |

No other file references this declaration — it is private and grants no
external visibility.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Names the `fmt` module for `MinorError`'s one trait impl |
