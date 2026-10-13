# 003: use core::fmt

## Representation

Brings the `core::fmt` module into scope under its own name, so the crate's
two `Display` implementations can write `fmt::Display`/`fmt::Formatter`/
`fmt::Result` instead of the fully qualified path. Private — not re-exported
— so it has no existence outside this file.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:41`

```rust
use core::fmt;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 122,124,135,137,421,430,450,458,506-508,513,517,543,752,754,757,885,887,890 | Every `fmt::` reference in the 4 `Display` impls (`KindError`, `Decimal`, `Qty`, `PriceAt`) and in `render` — `Formatter`, `Result`, `Error`, `Alignment`, and `Write::write_char` for the fill and the sign |

No other file references this declaration — it is private and grants no
external visibility.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The only crate that can see this binding; not re-exported |
