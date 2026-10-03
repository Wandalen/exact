# 003: use core::fmt

## Representation

Brings the `core::fmt` module into scope under its own name, so the crate's
two `Display` implementations can write `fmt::Display`/`fmt::Formatter`/
`fmt::Result` instead of the fully qualified path. Private — not re-exported
— so it has no existence outside this file.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:51`

```rust
use core::fmt;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 123,125,136,138,361,364,384,580,582 | Every `fmt::Display`/`fmt::Formatter`/`fmt::Result` reference in the 3 `Display` impls (`KindError`, `Decimal`, `Qty`) |

No other file references this declaration — it is private and grants no
external visibility.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The only crate that can see this binding; not re-exported |
