# 002: use exact_round::Rounding

## Representation

Brings in the three-variant rounding-mode enum that every `*_div_round`
function (and the private `div_round_minor` they share) takes as a
parameter and threads through to `exact_round::round_div`.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_ratio/src/lib.rs:34`

```rust
use exact_round::Rounding;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 34,171,186,199 | Parameter type of `div_round_minor` (private) and both public `*_div_round` functions |

A doc-comment mention at line 20 (`` [`exact_round::Rounding`] ``) is prose,
not a usage, and is excluded above.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Mode parameter for every rounding-division function |
