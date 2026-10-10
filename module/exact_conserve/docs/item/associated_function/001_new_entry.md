# 001: new for Entry

## Representation

Build a posting from any `Into<String>` account label, any `Into<String>`
asset, and a signed minor-unit amount.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:109-113`

```rust
pub fn new( account : impl Into< String >, asset : impl Into< String >, amount_minor : i64 ) -> Self
{
  Self { account : account.into(), asset : asset.into(), amount_minor }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 110-113 | Declaration |
| `src/lib.rs` | 215,218 | `verify`'s own doc-test |
| `tests/conservation_test.rs` | 20,52,58,67,82,91,105,120-123,138-141,166-169 | `transfer` helper, and the single- and multi-asset logs |
| `exchange_core/src/lib.rs:460-461` | — | **Production** |
| `cluster_economy/src/market.rs:504-505,515-516` | — | **Production** |
| `cluster_economy/tests/economy_test.rs:255-261` | — | Reconciliation assertion setup |
| `smoke_exact_market_split/src/lib.rs:127-128` | — | Demo-lane ledger postings |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised throughout its own tests and doc-test |
| `exchange_core` | `src/lib.rs` | **Production** — settlement postings |
| `cluster_economy` | `src/market.rs` | **Production** — settlement postings |
| `smoke_exact_market_split` | `src/lib.rs` | Demo-lane postings |

## Caller Tree

- **External:** `exchange_core::<settlement path>` (`exchange_core/src/lib.rs:460-461`)
- **External:** `cluster_economy::market::<settlement path>` (`cluster_economy/src/market.rs:504-505,515-516`)
- **External:** `smoke_exact_market_split::<ledger path>` (`smoke_exact_market_split/src/lib.rs:127-128`)

No intra-crate caller — `Entry::new` is a leaf constructor within
`exact_conserve` itself. The crate's most externally-called Item by a wide
margin: real production callers in two independent downstream crates, not
counting the demo lane or either crate's own tests.

## Callee Tree

- **External:** `Into::into` (the generic `account` parameter's conversion into `String`)
