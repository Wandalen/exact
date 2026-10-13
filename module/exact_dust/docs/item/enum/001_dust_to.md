# 001: DustTo

## Representation

Where the remainder of a split — equal or weighted — goes: folded into the first part
(`First`), held back and queried separately (`Sink`), or refused outright
(`Reject`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_dust/src/lib.rs:65-77`

```rust
/// Where the remainder of a split goes.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DustTo
{
  /// Folded into the first part.
  First,
  /// Held back — not included in any output slot; query it separately via
  /// [`money_dust_remainder`]/[`qty_dust_remainder`], or, for a weighted
  /// split, as the total minus the sum of the returned parts.
  Sink,
  /// A nonzero remainder is refused outright.
  Reject,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 66-77 | Declaration |
| `src/lib.rs` | 142,144,148,163,180 | `slot_minor`'s, `split_with`'s and `split_into_with`'s parameter, and `slot_minor`'s match scrutinee (private — no Item Instance of its own) |
| `src/lib.rs` | 210,221,246,256,324 | Parameter on the 4 `*_split`/`*_split_into` functions and `money_dust_split_weighted` (absent from the 2 `*_remainder` functions, which have no `to` parameter — the remainder is reported, never redirected) |
| `tests/dust_split_test.rs` | — | All 3 variants exercised across every split scenario |
| `exact_arith/src/lib.rs:140` | — | Facade re-export |
| `exact_arith/src/lib.rs:140` | — | Doctest import (crate-level `//! ``` ` example, compiled/run under `cargo test --doc`) |
| `smoke_exact_market_split/src/lib.rs:144` | — | `market_split`'s own call always passes `DustTo::First` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Selects remainder handling on every split; exercised by all of this crate's own tests |
| `exact_arith` | `src/lib.rs` | Re-export, and the facade's own crate-doc example constructs `DustTo::First` |
| `smoke_exact_market_split` | `src/lib.rs` | **Production** — `market_split` always passes `DustTo::First`, reached through the `exact_arith` facade re-export |
