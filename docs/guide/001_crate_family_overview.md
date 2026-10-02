# Guide: Crate Family Overview

### Scope

- **Purpose**: Let someone new to this repository learn, in one pass, what each crate does and how the 16 crates depend on each other.
- **Responsibility**: The real, current state of the family as implemented in `module/` today — not the original proposal.
- **In Scope**: One-line purpose per crate, the real dependency graph (verified against each crate's own `Cargo.toml`), the tier layering the crates themselves already use, and a suggested reading order.
- **Out of Scope**: Struct/enum/function-level API surface (→ [`../type/`](../type/readme.md), or each crate's own `docs/item/`); per-crate design rationale and disclosed deviations from the original proposal (→ each crate's own `docs/decisions/`); the original 15-crate proposal's own dependency tree (→ [`../readme.md`](../readme.md) § Dependency Tree, [`../crate/`](../crate/readme.md)).

**Design status**: current as of 2026-10-01, against the 16 crates that exist in `module/` today.

### Overview

This family gives the rest of the codebase exact, non-floating-point money
and quantity arithmetic: fixed-point decimals, checked/saturating operations,
explicit rounding, and a conservation audit — decomposed into small,
single-responsibility crates rather than one large one, so a consumer can
depend on exactly the operations it needs, or on `exact_arith` for all of
them at once.

Sixteen crates total: fifteen library crates arranged in five dependency
tiers (0 is lowest, depending on nothing in the family), plus one smoke-test
crate that exercises the whole family end to end through the facade alone.

### Quickstart

```rust
use exact_kind::{ Quantity, Money };

let held = Quantity::from_int( 3 ).unwrap();
let taken = Quantity::from_int( 5 ).unwrap();
assert!( held.checked_sub( taken ).is_err() );

let a = Money::parse( "0.1" ).unwrap();
let b = Money::parse( "0.2" ).unwrap();
assert_eq!( a.checked_add( b ).unwrap(), Money::parse( "0.3" ).unwrap() );
```

(from [`exact_kind`'s own readme](../../module/exact_kind/readme.md) — the family's
two headline guarantees in four lines: `0.1 + 0.2` stays exact, and a quantity can't
go negative.)

### Per-Crate Purpose

| Tier | Crate | Purpose |
|------|-------|---------|
| 0 | [`exact_minor`](../../module/exact_minor/readme.md) | The raw subunit integer and its checked/saturating arithmetic — no scale, no sign policy, no kind |
| 0 | [`exact_scale`](../../module/exact_scale/readme.md) | Scale-factor math and the declared range ceiling: the power-of-ten table and headroom |
| 0 | [`exact_round`](../../module/exact_round/readme.md) | Rounding modes and the family's default rounding policy |
| 1 | [`exact_sign`](../../module/exact_sign/readme.md) | Sign classification and the negative-value admission policy `exact_kind` enforces through |
| 1 | [`exact_kind`](../../module/exact_kind/readme.md) | The conserved value types — `Money`, `Qty`, `Price` — each a fixed-point decimal carrying its own scale |
| 2 | [`exact_add`](../../module/exact_add/readme.md) | Checked and saturating add/subtract, dispatched per kind |
| 2 | [`exact_ratio`](../../module/exact_ratio/readme.md) | A rational multiplier (`Ratio`) and mode-driven integer division, per kind |
| 2 | [`exact_parse`](../../module/exact_parse/readme.md) | Text parsing (`"1.23"` → minor units or error), per kind |
| 2 | [`exact_fmt`](../../module/exact_fmt/readme.md) | Formatting per kind, plus an allocation-free buffer-writing primitive |
| 2 | [`exact_bytes`](../../module/exact_bytes/readme.md) | `Wire`, a fixed-width byte encoding per kind, and its to/from conversions |
| 2 | [`exact_snap`](../../module/exact_snap/readme.md) | Snapping a price onto a `Tick` grid or a quantity onto a `Lot` grid |
| 2 | [`exact_cmp`](../../module/exact_cmp/readme.md) | Comparison, equality, and min/max per kind, with no epsilon |
| 3 | [`exact_dust`](../../module/exact_dust/readme.md) | Splitting a value into equal parts under a rounding mode, with the remainder ("dust") given an explicit destination |
| 3 | [`exact_conserve`](../../module/exact_conserve/readme.md) | Conservation auditing — a plain-log verifier plus a typed per-kind convenience layer |
| 4 | [`exact_arith`](../../module/exact_arith/readme.md) | The facade — re-exports all 14 leaf crates so a consumer depends on one crate instead of fourteen |
| 5 | [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) | The family's slice run end to end through `exact_arith` alone, with a floating-point control arm required to disagree |

### Dependency Tree

Verified directly against each crate's own `Cargo.toml` — not the original
proposal (→ [`../readme.md`](../readme.md) § Dependency Tree for that,
including the full account of where the real family deviates from it):

```text
exact_minor
exact_scale
exact_round
exact_sign                  → exact_minor
exact_kind                  → exact_minor, exact_scale
exact_add                   → exact_kind, exact_sign
exact_ratio                 → exact_kind, exact_round
exact_parse                 → exact_kind, exact_scale
exact_fmt                   → exact_kind
exact_bytes                 → exact_kind, exact_scale
exact_snap                  → exact_kind, exact_round
exact_cmp                   → exact_kind
exact_dust                  → exact_kind, exact_round
exact_conserve              → exact_add, exact_kind
exact_arith                 → all 14 crates above (facade; every edge direct, none transitive-only)
smoke_exact_market_split    → exact_arith
```

Roots (no dependencies): `exact_minor`, `exact_scale`, `exact_round`.
`exact_kind` is the one crate every tier-2, tier-3, and tier-4 crate depends
on directly — the only exception is the tier-5 smoke crate, which reaches it
solely through `exact_arith`.

One edge differs from the original proposal: `exact_dust` depends on
`exact_round`, not `exact_ratio` as first proposed. `exact_snap` already
depends on `exact_round` directly for the same `round_div` this crate needs,
and none of `exact_ratio`'s rational-multiplier surface is used — disclosed
in full in
[`exact_dust`'s own decision record](../../module/exact_dust/docs/decisions/001_direct_exact_round_dependency.md).

### Where to Start Reading

1. **`exact_kind`** — the three value types (`Money`, `Qty`, `Price`) everything else operates on.
2. **`exact_add`**, **`exact_cmp`** — the two operations used almost everywhere a value is handled.
3. **`exact_arith`** — the facade; for most consumers this is the only crate to depend on directly (`use exact_arith::{ Money, Quantity, /* … */ };`).
4. Everything else, on demand: parsing (`exact_parse`), formatting (`exact_fmt`), wire encoding (`exact_bytes`), grid snapping (`exact_snap`), equal-parts splitting (`exact_dust`), conservation auditing (`exact_conserve`).
5. **Run the demo** — `cargo run -p smoke_exact_market_split` runs the whole family end to end through `exact_arith` alone, with a floating-point control arm required to disagree, and prints a golden result.

### Related

- [`../readme.md`](../readme.md) § Dependency Tree — the original 15-crate proposal's own dependency tree, and the full account of where the real family deviates from it
- [`../crate/`](../crate/readme.md) — per-crate dependency/boundary specs as originally proposed
- [`../type/`](../type/readme.md) — per-crate struct/enum/function surface as originally proposed
- [`../../readme.md`](../../readme.md) — the family's own root readme and Responsibility Table
