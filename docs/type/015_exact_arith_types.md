# Type: exact_arith Types

### Scope

- **Purpose**: Specify `exact_arith`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: Re-exports of the 14 leaf crates' types and functions, plus 3 zero-constructors.
- **In Scope**: The functions this facade would define or re-export.
- **Out of Scope**: Its dependency edges (→ `../crate/015_exact_arith.md`); the old, differently-scoped `exact_arithmetic` facade this proposal was once contrasted against — deleted at the migration's cutover and no longer on disk, retrievable only via `git show HEAD:exact_arithmetic/<path>`.

**Design status**: implemented in [`exact_arith`](../../module/exact_arith/readme.md). Diverges from this proposal — the real facade defines no `exact_zero_money`/`exact_zero_qty`/`exact_zero_price` functions; each would duplicate a constant the facade already re-exports (`Money::ZERO`, `Quantity::ZERO`, `Price::ZERO`), so it declares nothing of its own at all, matching `exact_arith`'s facade-purity invariant — see [`exact_arith`'s own invariant doc](../../module/exact_arith/docs/invariant/001_facade_re_exports_only.md) for the full account.

### Functions

- Re-exports of the leaf types and functions from all 14 other proposed crates.
- `exact_zero_money`, `exact_zero_qty`, `exact_zero_price` — the only functions this crate defines itself.

### Errors

None — "no new policy" beyond the leaves.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:504-507` | `exact_arith`'s full re-export/function listing |
