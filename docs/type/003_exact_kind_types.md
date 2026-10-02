# Type: exact_kind Types

### Scope

- **Purpose**: Specify `exact_kind`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `Money`/`Qty`/`Price` newtypes, their conversions, and the `Scaled` trait.
- **In Scope**: Structs, functions, the error enum, and the trait this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/003_exact_kind.md`).

**Design status**: implemented in [`exact_kind`](../../module/exact_kind/readme.md), with deviations across every section of this proposal — see [its own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) and [non-negativity decision](../../module/exact_kind/docs/decisions/001_non_negativity_enforced_at_construction.md) for the full account. `Money`/`Qty`/`Price` are not three independent `{ minor, scale }` structs: the real type is one `Decimal<const SCALE: u32> { minor: Backing }` carrying scale as a compile-time const generic rather than a runtime field, with `Money` and `Price` as plain aliases of `Decimal<MONEY_SCALE>` (identical types, not yet distinguished — no real consumer has required it) and `Quantity` a separate `Qty<SCALE>` wrapper that refuses negative values. Construction/extraction/scale-access are inherent methods (`from_minor`, `from_int`, `parse`, `minor()`), not free functions named `money_from_minor`/`money_to_minor`/`money_scale` per kind. `KindError` has no `ScaleMismatch` (two different `SCALE`s are different Rust types and can never reach a shared function — the compiler already refuses it) but adds `ExceedsCeiling`, `ExcessPrecision`, `Malformed`, and `Negative` this proposal didn't name. No `Scaled` trait was built — it would return a runtime `Scale` this family's compile-time-const design doesn't have.

### Structs

- `Money { minor, scale }`
- `Qty { minor, scale }`
- `Price { minor, scale }`

### Functions

- `money_from_minor`, `qty_from_minor`, `price_from_minor` — construction.
- `money_to_minor`, `qty_to_minor`, `price_to_minor` — extraction.
- `money_scale`, `qty_scale`, `price_scale` — scale access.

### Errors

- `KindError { ScaleMismatch }`

### Traits

- `Scaled { fn scaled_minor(&self) -> Minor; fn scaled_scale(&self) -> Scale; }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:440-448` | `exact_kind`'s full struct/function/error/trait listing |
