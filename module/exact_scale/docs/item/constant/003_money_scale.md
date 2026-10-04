# 003: MONEY_SCALE

## Representation

The scale the family's money-like values are expressed at — 6 places. The
usual settlement precision for a currency whose smallest physical unit is two
places, leaving four places of sub-cent room for per-unit prices.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_scale/src/lib.rs:37`

```rust
pub const MONEY_SCALE : u32 = 6;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 37,45 | Declaration; `CEILING_MINOR_UNITS`'s own definition |
| `tests/scale_factor_test.rs:3,21` | — | Cross-check |
| `exact_parse/src/lib.rs:36` | — | **Production** — compile-time consistency assert against `Money::ONE_MINOR` |
| `exact_bytes/src/lib.rs:35,165,181,192,209,220,234` | — | **Production** — the wire scale byte every `Wire` carries, and the round-trip scale check on every `*_from_wire` |
| `exact_arith/src/lib.rs:81` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:52,53,54` | — | Direct value check and cross-checks via the re-exported name |
| `exact_kind/src/lib.rs:57,632,60` | — | **Production** — fixes `Money`/`Price`/`Quantity`'s type-level scale |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_scale` | `(defining crate)` | Defines `CEILING_MINOR_UNITS`; exercised by its own cross-check test |
| `exact_parse` | `src/lib.rs` | **Production** — compile-time cross-check, not a runtime dependency |
| `exact_bytes` | `src/lib.rs` | **Production** — the standard scale byte in the wire format, and its round-trip validation |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; its own test checks the value directly |
| `exact_kind` | `src/lib.rs` | **Production** — the single most consumed item from `exact_scale`: it fixes all three of the family's standard type aliases |
