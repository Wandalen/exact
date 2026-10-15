# ADR-001: Dependency Contract Retired For The Typed Layer

**Date**: 2026-10-01
**Status**: Accepted
**Deciders**: wandalen

## Context

`exact_audit`'s own `docs/decisions/001_zero_dependency_by_contract.md` fixed that crate's manifest at no `[dependencies]` at all, enforced by a manifest-reading test (`the_manifest_declares_no_dependencies`), on the reasoning that a dependency-free auditor is testable against a log from any source — persisted, wired, or hand-written — never only live in-process values from one specific type family. The preferred design's own dependency-tree edge for this crate (`exact_conserve → exact_add, exact_kind`) requires a typed convenience layer that Contract would forbid outright.

## Decision

`exact_conserve` depends on `exact_add` and `exact_kind`. The original zero-dependency Contract is retired for the crate as a whole — but `Entry`, `Report`, and `verify` themselves are still written to touch neither dependency, so the Contract's actual engineering value (a plain-log auditor testable independent of any one value-type family) survives inside those three items even though the manifest they ship in is no longer empty. `the_manifest_declares_no_dependencies` is dropped rather than ported, since it would now test a property this crate deliberately no longer holds as a whole.

## Alternatives Considered

### Option 1: Keep the zero-dependency manifest and put the typed layer in a separate crate

Ship `money_conserve_into`/`qty_conserve_into`/`money_sum_assert_zero`/`qty_sum_assert_zero` from a new crate layered on top of a still-dependency-free `exact_conserve`. Rejected: the preferred design names this crate, not a further split, as the home for the typed layer, and a family already at sixteen crates gains a seventeenth for a handful of thin wrapper functions — a cost not offset by any benefit a split would uniquely provide.

### Option 2: Drop the Contract silently, with no record

Just add the dependencies and move on, since the preferred design already names the edge. Rejected: a reader who finds `exact_audit`'s old ADR (via git history) and then finds `exact_conserve` depending on exactly what that ADR forbade would reasonably wonder whether the constraint was abandoned by oversight or by decision. Recording the reversal, and what of the original reasoning still holds, is cheaper than leaving that question open.

## Consequences

**Positive:**
- The typed layer gets real, reusable checked arithmetic (`exact_add::money_add`/`qty_add`) instead of a second, locally-written copy of it.
- `Entry`, `Report`, and `verify` remain exactly as independently testable as before — a log built from nothing but integers and strings still exercises the whole plain-log path with no typed value in sight (see `a_log_can_be_built_from_nothing_but_integers`, `tests/conservation_test.rs`).

**Negative:**
- The crate as a whole can no longer claim, as `exact_audit` could, that its entire public surface is reachable with zero dependencies — only the three plain-log items can still make that claim, and a reader has to know to look at the function level rather than the manifest to find it.

**Neutral:**
- `exact_add` and `exact_kind` are themselves small, single-concern crates at a lower tier than `exact_conserve` — adopting them is not the start of a wider dependency creep, and no crate in the family depends on `exact_conserve` for anything the original Contract's reasoning would need to protect.

## Related

- [Entry And Report](../type/001_entry_and_report.md) — `Entry`/`Report`, unchanged by this reversal, and `ConservationError`'s own shape
- [Conservation Verification Fold](../algorithm/001_conservation_verification_fold.md) — `verify`, the one surface this crate's dependency-free claim still covers in full, plus the typed layer this decision makes possible
