# 004: discrepancy_minor

## Representation

The signed discrepancy in minor units — zero when balanced. Signed
deliberately: the sign distinguishes value appearing from value vanishing,
which the doc comment calls "different investigations" (`src/lib.rs:166-167`).

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:169-172`

```rust
pub const fn discrepancy_minor( &self ) -> i128
{
  self.net_minor
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 169-172 | Declaration |
| `src/lib.rs` | 199 | `verify`'s own doc-test |
| `tests/conservation_test.rs:31,53,56,69,90` | — | Asserts the exact signed leftover in 5 distinct scenarios |
| `exchange_core/tests/submission_test.rs:259` | — | Integration test |
| `smoke_exact_market_split/src/lib.rs:199`, `tests/lane_test.rs:65` | — | Demo-lane leak-magnitude assertion |

Confirmed via grep across the full workspace (not only `exact_conserve`'s
direct dependents): no production call site in `cluster_economy` or
`exchange_core`'s own non-test source — both consume `is_balanced` in
production but read the discrepancy amount, where they need it at all, only
in their own tests. A real, verified asymmetry between these two sibling
methods, not an oversight in this catalog.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised across 5 distinct scenarios in its own tests |
| `exchange_core` | `tests/submission_test.rs` | Integration test |
| `smoke_exact_market_split` | `src/lib.rs`, `tests/lane_test.rs` | Demo-lane leak-magnitude assertion |

## Caller Tree

- **External:** `exchange_core`'s own integration test (`tests/submission_test.rs:259`)
- **External:** `smoke_exact_market_split`'s own demo-lane code and test (`src/lib.rs:199`, `tests/lane_test.rs:65`)

No intra-crate caller, and no confirmed production (non-test, non-demo-lane)
caller — narrower reach than its sibling `is_balanced`, which `cluster_economy`
does call from real settlement code.

## Callee Tree

No callee of its own — reads `self.net_minor` directly; no further hop into
another Item Instance.
