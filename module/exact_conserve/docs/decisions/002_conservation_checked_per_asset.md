# ADR-002: Conservation Is Checked Per Asset

**Date**: 2026-10-10
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`verify` summed every posting in a log into one total, carried forward from
`exact_audit`. That is right for a log of one asset, and wrong for a log of
several: a trade moves cash one way and an instrument the other, and one total
adds units of different things together. A log that invents one unit of cash
and loses one unit of an instrument sums to zero and reports balanced:

| Posting | cash | BTC |
|---------|-----:|----:|
| buyer pays | −100 | |
| seller receives | +101 | |
| seller delivers | | −5 |
| buyer receives | | +4 |
| **one total** | **0** | |
| **per asset** | **+1** | **−1** |

A caller could split the log by asset and call `verify` once per asset — the
`verify` item page records `cluster_economy` doing exactly that, auditing a
settlement's cash legs and asset legs separately — but nothing made a caller
do it, and the exchange built on this family posts only its cash legs.

`qty_sum_assert_zero` was the typed layer's answer for quantities, and it is
the wrong shape for the job: every `Quantity` leg is non-negative, so the sum
is zero only when every leg is, and a quantity *movement* — the seller's −5 —
cannot be expressed as a `Quantity` at all.

## Decision

- `Entry` gains `asset : A`, naming what moved — a currency or an
  instrument — and `Entry::new` takes it between the account and the amount.
  `A` is the caller's own key type: an enum, where a misspelt asset is a
  compile error, or a `&str`/`String` where text is enough.
- `verify< A : Ord + Clone >` keeps one `i128` net per asset, in a
  `BTreeMap< A, i128 >`, and never adds amounts of different assets together.
  It clones a key only the first time it sees that asset.
- `Report` carries those nets as `nets` in place of the single `net_minor`.
  `is_balanced` holds when every net is zero, and `discrepancy_minor( asset )`
  returns `Some` of one asset's net, or `None` for an asset the log never
  moved — so a misspelt asset cannot read as a zero discrepancy.
- `Report`'s `Display` keeps its balanced text, and when unbalanced names each
  asset whose net is not zero, in asset order:
  `UNBALANCED: entries 4, net BTC -1, cash 1 minor units`.
- `qty_sum_assert_zero` is removed, along with its `exact_arith` re-export. A
  quantity's movements are audited through `verify` with an asset key, as
  signed `i64` amounts. `money_sum_assert_zero` stays: money legs are signed,
  so they can cancel.

## Alternatives Considered

### Option 1: Keep one total, and leave per-asset auditing to the caller

Rejected: the caller has to know to split the log, and the auditor reports a
mixed log balanced when it is not. An auditor whose answer depends on the
caller remembering a rule is the gap this crate exists to close.

### Option 2: A `String` asset key

Rejected in review: any text is a valid `String`, so a misspelt asset is not
an error — `discrepancy_minor( "csah" )` would report the zero of an asset
never moved while `cash` is off — and `verify` cloned the key on every
posting. A key type the caller chooses lets an enum make the misspelling a
compile error and the clone free, while `&str` keys keep string literals
working.

### Option 3: A `HashMap` of nets

Rejected: a `HashMap` iterates in an order that changes from run to run, so
the same log would render its `Display` text, and its `Debug` output,
differently each time — and the tests and the smoke demo compare that text.
A log moves a handful of assets, where a `BTreeMap`'s ordered iteration costs
nothing measurable.

### Option 4: Deprecate `qty_sum_assert_zero` instead of removing it

Rejected: a deprecated function still compiles and still invites a check that
cannot check anything, and this change already breaks every caller of
`Entry::new` and `discrepancy_minor` — removing it in the same change costs a
caller nothing a deprecation would have spared them. No caller outside this
crate's own test was found.

## Consequences

**Positive:**
- A leak in one asset can no longer hide behind a forgery in another, and the
  report names each asset that is off, with the sign that says which way.
- One `verify` call audits a whole multi-asset log.

**Negative:**
- Breaking: every `Entry::new` call gains an argument, every
  `discrepancy_minor()` call an asset and an `Option` result, `Entry` and
  `Report` a type parameter, `Report` loses `Copy` and its
  `net_minor` field, and `qty_sum_assert_zero` is gone. Inside this workspace: `exact_conserve`'s own tests,
  `exact_arith`'s doc example and facade test, and `smoke_exact_market_split`.
  Outside it: the exchange's `postings`, and `cluster_economy`.
- `is_balanced` and `discrepancy_minor` are no longer `const fn` — both read a
  map, which `const` code cannot. No caller used either at compile time.
- `verify` now allocates one map entry per asset, and clones one key per
  asset — free for a `Copy` key such as an enum or a `&str`.

**Neutral:**
- An `Entry`'s `account` is still carried for reporting only; per-account
  totals are still not computed, for the reason the module doc gives.
- The typed layer's `money_*` functions are unchanged.

## Related

- [ADR-001](001_dependency_contract_retired_for_typed_layer.md) — `Entry`/`Report`/`verify` still touch neither of this crate's dependencies
- [Entry And Report](../type/001_entry_and_report.md) — the shapes this changes
- [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) — the per-asset fold
- [Sum-To-Zero Is Exact](../invariant/001_sum_to_zero_is_exact.md) — the invariant, now per asset
- [Feature 013](../../../../docs/feature/013_sum_assert_zero.md) — the requirement naming `qty_sum_assert_zero`; its Design status records the removal
