# Type: exact_fmt Types

### Scope

- **Purpose**: Specify `exact_fmt`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `fmt` per kind, buffer-writing `fmt_into`, and `Display` as a thin wrapper.
- **In Scope**: The functions and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/010_exact_fmt.md`).

**Design status**: implemented in [`exact_fmt`](../../module/exact_fmt/readme.md). `money_fmt`, `qty_fmt`, `price_fmt`, `fmt_into`, and `FmtError { BufFull }` all match this proposal's names exactly. The `Display` trait impl diverges — it could not move into this crate at all: `exact_fmt` depends on `exact_kind`, so under Rust's orphan rules this crate owns neither the foreign trait (`core::fmt::Display`) nor the foreign type (`Decimal`/`Qty`) needed to implement it here. `Display` instead stays on `exact_kind`'s own types (ported from the real codebase, then reworked not to allocate; `Price`'s delegates to `Money`'s), with this crate's functions rendering through it — see [`exact_fmt`'s algorithm doc](../../module/exact_fmt/docs/algorithm/001_decimal_rendering_per_kind.md) for the full account.

### Functions

- `money_fmt`, `qty_fmt`, `price_fmt` — formatting per kind.
- `fmt_into(buf: &mut [u8]) -> Result<usize, FmtError>` — the buffer-writing primitive.

### Errors

- `FmtError { BufFull }`

### Trait Impls

`Display` is implemented only as a wrapper over `fmt_into` — no independent formatting logic.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:479-483` | `exact_fmt`'s full function/error listing, including the `Display`-wraps-`fmt_into` note |
