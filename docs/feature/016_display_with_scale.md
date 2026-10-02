# Feature: Display With Scale

### Scope

- **Purpose**: Print a value using its own scale, while storage stays an integer.
- **Responsibility**: `exact_fmt`'s `_fmt`/`fmt_into` functions and `Display` wrapper.
- **In Scope**: `money_fmt`, `qty_fmt`, `price_fmt`, `fmt_into(buf) -> Result<usize, FmtError>`.
- **Out of Scope**: Parsing, the inverse direction (→ Feature 005, `exact_parse`).

**Design status**: implemented in [`exact_fmt`](../../module/exact_fmt/readme.md) — `money_fmt`/`qty_fmt`/`price_fmt` and `fmt_into(buf) -> Result<usize, FmtError>` all exist and match the proposal. Diverges on where `Display` itself lives: the proposal wants it implemented here "only as a wrapper over `fmt_into`," but Rust's orphan rules forbid `exact_fmt` from implementing the foreign `core::fmt::Display` trait for the foreign `exact_kind::Decimal`/`Qty` types, so `Display` stays on `exact_kind::Decimal`/`Qty` instead and this crate's functions dispatch to it — see `exact_fmt`'s own [rendering algorithm doc](../../module/exact_fmt/docs/algorithm/001_decimal_rendering_per_kind.md) for the full account.

### Statement

Display is a thin, non-allocating wrapper (`fmt_into`) over the stored integer and its scale. Storage never changes shape to accommodate printing — "1.10" is a rendering of `110` at scale 2, not a stored string.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:196` | Feature 16 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:108-114` | Hard problem 8, "Display is not storage," this feature addresses |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:479-483` | `exact_fmt`'s proposed functions and `FmtError` |
