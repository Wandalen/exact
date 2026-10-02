# Hard Problem: Display Is Not Storage

### Scope

- **Purpose**: State why printing and storage must stay separate concerns.
- **Responsibility**: The requirement that storage remain an integer regardless of how a value is displayed.
- **In Scope**: All display and storage paths.
- **Out of Scope**: The display function itself (→ `../feature/016_display_with_scale.md`).

**Design status**: implemented in [`exact_fmt`](../../module/exact_fmt/readme.md) (`money_fmt`/`qty_fmt`/`price_fmt`, `fmt_into`) over `exact_kind`'s integer-backed `Decimal`/`Qty` storage — holds as stated. One placement deviation: the proposal wanted `Display` implemented in `exact_fmt` itself "only as a wrapper over `fmt_into`," but Rust's orphan rules forbid `exact_fmt` from implementing the foreign `core::fmt::Display` trait for `exact_kind`'s foreign types, so `Display` lives on `exact_kind::Decimal`/`Qty` instead and `exact_fmt`'s functions dispatch to it — see [`exact_fmt`'s own rendering algorithm doc](../../module/exact_fmt/docs/algorithm/001_decimal_rendering_per_kind.md) for the full account.

### Statement

- **Purpose**: store an integer; print with a scale.
- **Why**: "1.10" parsed as f64 reintroduces problem 1.
- **If missing**: JSON floats in the save.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:108-114` | Hard problem 8, verbatim Purpose/Why/If-missing |
