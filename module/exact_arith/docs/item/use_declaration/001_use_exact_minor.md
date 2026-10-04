# 001: pub use exact_minor::{ ... }

## Representation

Re-exports `exact_minor`'s entire public surface — the raw `Backing` type
alias and its checked/saturating arithmetic — one of the 3 Tier-0 roots this
facade depends on directly despite none of its own re-exported functions
taking `Backing` by name in their signature (module doc comment,
`src/lib.rs:47-56`: a `pub use` requires a direct dependency regardless of
whether the type is reachable transitively).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:65-79`

```rust
pub use exact_minor::
{
  Backing,
  Minor,
  MinorError,
  minor_checked_add,
  minor_checked_neg,
  minor_checked_sub,
  minor_from_i64,
  minor_is_zero,
  minor_saturating_add,
  minor_saturating_sub,
  minor_to_i64,
  minor_zero,
};
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 65-79 | Declaration |
| `exchange_core/src/lib.rs:63-66` | — | **Production** — re-exports `Backing` one layer further, to its own consumers |
| `exchange_types/src/lib.rs:35` | — | **Production** — imports `Backing` directly |

**Not exercised by this crate's own test suite at all** (`tests/facade_test.rs`
imports none of these 12 names) — the one re-export block in this crate
confirmed to have real production reach through the facade while remaining
completely untested at the facade layer itself. See readme Notable Findings
for the full 3-way split across all 14 blocks.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `(defining crate)` | Declared only — not exercised by this crate's own tests |
| `exchange_core` | `src/lib.rs` | **Production** — re-exported one layer further |
| `exchange_types` | `src/lib.rs` | **Production** — `Backing` used directly |
