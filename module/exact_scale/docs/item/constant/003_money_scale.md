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
| `exact_parse/src/lib.rs:46` | — | **Production** — compile-time consistency assert against `Money::ONE_MINOR` |
| `exact_bytes/tests/wire_roundtrip_test.rs:8,45,48,59,89,97,100,110,111,119,120,121,138` | — | Test only — names the scale a `*_from_wire` call decodes at, where nothing else fixes it; the wire's scale byte itself is the encoded type's own `S` |
| `exact_parse/tests/from_str_test.rs:7,13,14,15,22,29,30,31,32,33,42,43,44,45,53,54,63` | — | Test only — names the scale each `*_from_str` call parses at |
| `exact_snap/src/lib.rs:26,79,117` | — | **Production** — fixes the `Tick`/`Lot` aliases' scale |
| `exact_arith/src/lib.rs:82` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:52,53,54` | — | Direct value check and cross-checks via the re-exported name |
| `exact_kind/src/lib.rs:55,58,61,64` | — | **Production** — fixes `Money`/`Quantity`/`Price`'s type-level scale |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_scale` | `(defining crate)` | Defines `CEILING_MINOR_UNITS`; exercised by its own cross-check test |
| `exact_parse` | `src/lib.rs` | **Production** — compile-time cross-check, not a runtime dependency |
| `exact_bytes` | `tests/wire_roundtrip_test.rs` | Dev-dependency only — the scale a test decodes at |
| `exact_parse` | `tests/from_str_test.rs` | The scale a test parses at, beside the production cross-check above |
| `exact_snap` | `src/lib.rs` | **Production** — the scale the `Tick`/`Lot` aliases are fixed at |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; its own test checks the value directly |
| `exact_kind` | `src/lib.rs` | **Production** — the single most consumed item from `exact_scale`: it fixes all three of the family's standard type aliases |
