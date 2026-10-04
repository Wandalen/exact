# 002: pub use exact_scale::{ ... }

## Representation

Re-exports `exact_scale`'s 4 named constants and `pow10` — the second
Tier-0 root.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:81`

```rust
pub use exact_scale::{ CEILING_MINOR_UNITS, CEILING_WHOLE_UNITS, HEADROOM_FACTOR, MONEY_SCALE, pow10 };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 81 | Declaration |
| `tests/facade_test.rs:10-11,52-54` | — | `MONEY_SCALE`, `pow10`, `CEILING_MINOR_UNITS`, `CEILING_WHOLE_UNITS` all asserted against each other |
| `exchange_types/src/lib.rs:35` | — | **Production** — `MONEY_SCALE`, `pow10` |
| `exchange_escrow/tests/reservation_test.rs:8` | — | `CEILING_WHOLE_UNITS` |

`HEADROOM_FACTOR` has no confirmed caller anywhere through this facade,
intra-crate or external — re-exported per the module doc comment's disclosed
"generous precedent" (`src/lib.rs:57-63`) rather than because a consumer
needs it today.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `tests/facade_test.rs` | **Exercised** — cross-checks the scale/ceiling relationship |
| `exchange_types` | `src/lib.rs` | **Production** |
| `exchange_escrow` | `tests/reservation_test.rs` | Test only |
