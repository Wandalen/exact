# 001: DustTo

## Representation

Where the remainder of an equal split goes: folded into the first part
(`First`), held back and queried separately (`Sink`), or refused outright
(`Reject`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_dust/src/lib.rs:50-61`

```rust
/// Where the remainder of an equal split goes.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DustTo
{
  /// Folded into the first part.
  First,
  /// Held back — not included in any output slot; query it separately via
  /// [`money_dust_remainder`]/[`qty_dust_remainder`].
  Sink,
  /// A nonzero remainder is refused outright.
  Reject,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 51-61 | Declaration |
| `src/lib.rs` | 121,123,130 | `fill_minor`'s parameter and match scrutinee (private — no Item Instance of its own) |
| `src/lib.rs` | 151,166,196,210 | Parameter on the 4 `*_split`/`*_split_into` functions (absent from the 2 `*_remainder` functions, which have no `to` parameter — the remainder is reported, never redirected) |
| `tests/dust_split_test.rs` | — | All 3 variants exercised across every split scenario |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |
| `exact_arith/src/lib.rs:127` | — | Doctest import (crate-level `//! ``` ` example, compiled/run under `cargo test --doc`) |
| `smoke_exact_market_split/src/lib.rs:134` | — | `market_split`'s own call always passes `DustTo::First` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Selects remainder handling on every split; exercised by all of this crate's own tests |
| `exact_arith` | `src/lib.rs` | Re-export, and the facade's own crate-doc example constructs `DustTo::First` |
| `smoke_exact_market_split` | `src/lib.rs` | **Production** — `market_split` always passes `DustTo::First`, reached through the `exact_arith` facade re-export |
