# Type: exact_conserve Types

### Scope

- **Purpose**: Specify `exact_conserve`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `sum_assert_zero` per kind and the `conserve_into` accumulator.
- **In Scope**: The functions and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/013_exact_conserve.md`).

**Design status**: implemented in [`exact_conserve`](../../module/exact_conserve/readme.md). `ConservationError`'s two variants match as specified (`NotZero { got }`, `Overflow`); the proposal's single generic `conserve_into(acc, leg)` is instead two per-kind-prefixed functions, `money_conserve_into`/`qty_conserve_into`, matching this family's established per-kind naming convention — see [`exact_conserve`'s own algorithm doc](../../module/exact_conserve/docs/algorithm/001_conservation_verification_fold.md). Beyond this listing, the plain-log auditor (`Entry`, `Report`, `verify`) nets each asset separately — an `Entry` names its `asset`, and a `Report` holds one net per asset — and `qty_sum_assert_zero` is not shipped, in its favour — a slice of non-negative quantities sums to zero only when every leg is zero ([ADR-002](../../module/exact_conserve/docs/decisions/002_conservation_checked_per_asset.md)).

### Functions

- `money_sum_assert_zero`, `qty_sum_assert_zero` — the conservation assertion per kind.
- `conserve_into(acc, leg)` — accumulator-style folding.

### Errors

- `ConservationError { NotZero { got }, Overflow }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:495-498` | `exact_conserve`'s full function/error listing |
