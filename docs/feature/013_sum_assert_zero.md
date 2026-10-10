# Feature: Sum Assert Zero

### Scope

- **Purpose**: Fold a slice of ledger legs and assert the result is exactly zero.
- **Responsibility**: `exact_conserve`'s `_sum_assert_zero` functions.
- **In Scope**: `money_sum_assert_zero`, `qty_sum_assert_zero`, `conserve_into`.
- **Out of Scope**: The typed error the fold raises on failure (→ Feature 014).

**Design status**: implemented in [`exact_conserve`](../../module/exact_conserve/readme.md) — `money_sum_assert_zero` matches the proposal exactly. `qty_sum_assert_zero` is not shipped: every `Quantity` leg is non-negative, so a slice of them sums to zero only when every leg is zero, and it could never check a transfer, whose giving side is negative — a quantity's movements are audited through `verify` with an asset key instead, as signed amounts — see [`exact_conserve`'s ADR-002](../../module/exact_conserve/docs/decisions/002_conservation_checked_per_asset.md). Diverges on `conserve_into`: it is split into per-kind `money_conserve_into`/`qty_conserve_into` rather than one generic `conserve_into(acc, leg)`, matching this family's established per-kind naming convention (`exact_add`, `exact_ratio`, `exact_cmp`, …) — see `exact_conserve`'s own [conservation verification fold algorithm](../../module/exact_conserve/docs/algorithm/001_conservation_verification_fold.md) for the full account.

### Statement

`sum_assert_zero` is the audit primitive: fold a slice of legs and assert the net is zero, giving the family an actual audit tool rather than an implicit hope that debits and credits balance.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:190` | Feature 13 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:354-362` | `exact_conserve`'s proposed responsibility and boundary |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:495-498` | `exact_conserve`'s proposed functions and `ConservationError` |
