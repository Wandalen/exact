# Hard Problem: Determinism

### Scope

- **Purpose**: State why the same operation must produce the same bits everywhere.
- **Responsibility**: The requirement of cross-platform bit-for-bit reproducibility.
- **In Scope**: All arithmetic and comparison operations.
- **Out of Scope**: The comparison functions that depend on this (→ `../feature/011_exact_eq_and_ord_no_epsilon.md`).

**Design status**: implemented in [`exact_cmp`](../../module/exact_cmp/readme.md) (`money_cmp`/`money_eq`, `qty_cmp`, `price_cmp`/`price_min`/`price_max`), dispatching to bit-exact integer comparison with no tolerance window. Holds as stated, and more strongly than proposed: there is no `CmpError`/runtime scale guard, because this family's compile-time `SCALE` generic makes a cross-scale comparison a compile error no runtime check could ever reach — see [`exact_cmp`'s own decision](../../module/exact_cmp/docs/decisions/001_no_cmp_error_unconditional_ord.md). The guarantee is structural across the whole family: every value is an integer (`i64`/`i128`) with no floating-point anywhere, so the same operation produces the same bits on every platform.

### Statement

- **Purpose**: same operations, same bits, every platform.
- **Why**: two nodes must agree on a fee.
- **If missing**: desync on the first split.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:100-106` | Hard problem 7, verbatim Purpose/Why/If-missing |
