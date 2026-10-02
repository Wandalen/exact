# Hard Problem: Closed Vm Types

### Scope

- **Purpose**: State why a balance must be plain bits with no host heap pointer.
- **Responsibility**: The requirement that values be snapshot- and relocation-safe.
- **In Scope**: Every kind's in-memory representation.
- **Out of Scope**: The ECS crate (001) this requirement lets these types live inside — that crate is entirely outside this family.

**Design status**: satisfied by construction in [`exact_kind`](../../module/exact_kind/readme.md) — no dedicated crate or decision was needed. `Decimal<SCALE>` is one `minor: Backing` field and `Qty<SCALE>` is one `value: Decimal<SCALE>` field, both deriving `Copy` with no heap allocation or pointer anywhere in the representation, so a `Money`/`Qty`/`Price` value is already plain relocatable bits — see [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) (its Sources table cites the exact struct definitions).

### Statement

- **Purpose**: a balance is plain bits, no host heap pointer.
- **Why**: a wallet row must snapshot and relocate.
- **If missing**: 006 cannot live inside 001.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:124-130` | Hard problem 10, verbatim Purpose/Why/If-missing |
