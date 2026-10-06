# 001: verify

## Representation

Audit a log of postings for conservation: fold every `amount_minor` into an
`i128`-widened net total, and report it. The crate's single most
production-critical export.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_conserve/src/lib.rs:205-215`

```rust
pub fn verify( entries : &[ Entry ] ) -> Result< Report, ConservationError >
{
  let mut net : i128 = 0;
  for entry in entries
  {
    net = net
    .checked_add( i128::from( entry.amount_minor ) )
    .ok_or( ConservationError::Overflow )?;
  }
  Ok( Report { entries : entries.len(), net_minor : net } )
}
```

Carries its own doc-test (`src/lib.rs:192-200`), which runs as a real
`cargo test --doc` execution, demonstrating both a balanced log and a
one-unit leak.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 205-215 | Declaration |
| `src/lib.rs` | 192-200 | Own doc-test |
| `tests/conservation_test.rs` | throughout | Every test in the file |
| `exact_arith/src/lib.rs:30` | — | Facade's own module-level doc-test |
| `exact_arith/tests/facade_test.rs:29` | — | Facade integration test |
| `cluster_economy/src/market.rs:507,518` | — | **Production** — audits a settlement's cash legs and asset legs separately before accepting it |
| `cluster_economy/tests/economy_test.rs:263-264` | — | Reconciliation assertions |
| `exchange_core/tests/submission_test.rs:257` | — | Integration test |
| `smoke_exchange_core/src/lib.rs:105` | — | Demo-lane settlement audit |
| `smoke_exact_market_split/src/lib.rs:308,313`, `tests/lane_test.rs:63,66` | — | Demo-lane ledger checks (incl. the ported `bug_reproducer` regression test) |

Confirmed via a full-workspace grep (not limited to `exact_conserve`'s direct
`Cargo.toml` dependents) — the same wider sweep that corrected this
catalog's first-draft miss on `is_balanced` (§ readme Notable Findings).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised throughout its own tests and doc-test |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Doc-test and integration test |
| `cluster_economy` | `src/market.rs` | **Production** — the settlement-path conservation gate |
| `exchange_core` | `tests/submission_test.rs` | Integration test |
| `smoke_exchange_core` | `src/lib.rs` | Demo-lane settlement audit |
| `smoke_exact_market_split` | `src/lib.rs`, `tests/lane_test.rs` | Demo-lane ledger checks |

## Caller Tree

- **External:** `cluster_economy::market::<settlement path>` (`cluster_economy/src/market.rs:507,518`) — the one confirmed production caller, immediately followed by an `is_balanced()` check on the returned `Report` (see [is_balanced](../associated_function/003_is_balanced.md))
- **External:** `exchange_core`, `smoke_exchange_core`, `smoke_exact_market_split` — integration tests and demo-lane code, not production

No intra-crate caller.

## Callee Tree

No callee of its own — folds via `i128::from`/`i128::checked_add` directly
and constructs [Report](../struct/002_report.md); no further hop into
another Item Instance.
