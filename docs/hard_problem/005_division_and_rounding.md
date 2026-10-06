# Hard Problem: Division And Rounding

### Scope

- **Purpose**: State why fees and splits need an explicit rounding mode.
- **Responsibility**: The requirement that division name its rounding mode rather than default it.
- **In Scope**: Fee and split arithmetic.
- **Out of Scope**: The rounding enum itself (→ `../feature/008_rounding_mode_enum.md`).

**Design status**: implemented in [`exact_round`](../../module/exact_round/readme.md) (the `Rounding` enum: `Down`/`Up`/`HalfEven`) and [`exact_ratio`](../../module/exact_ratio/readme.md) (`money_div_round`/`qty_div_round` and every `*_mul_ratio`, each taking the mode as a required argument). One real deviation: the documented default is `HalfEven`, not `Down` as originally proposed, chosen as the one mode with no directional bias over repeated roundings — see [`exact_round`'s own decision](../../module/exact_round/docs/decisions/001_half_even_as_the_unbiased_default.md) for the full account.

### Statement

- **Purpose**: fees and splits use an explicit mode (down, half-even, up).
- **Why**: each caller otherwise rounds differently.
- **If missing**: conservation breaks on the remainder.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:84-90` | Hard problem 5, verbatim Purpose/Why/If-missing |
