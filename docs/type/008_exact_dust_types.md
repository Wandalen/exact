# Type: exact_dust Types

### Scope

- **Purpose**: Specify `exact_dust`'s proposed full code surface, so its API shape is fixed before implementation.
- **Responsibility**: `DustTo`, `dust_split`, and the non-allocating `dust_split_into` variant.
- **In Scope**: The enum, functions, and error enum this crate would define.
- **Out of Scope**: Its dependency edges (→ `../crate/008_exact_dust.md`).

**Design status**: implemented in [`exact_dust`](../../module/exact_dust/readme.md). `DustTo { First, Sink, Reject }` and `DustError { EmptyParts, Remainder, Overflow }` match this proposal's names exactly; `Remainder` is also what `Rounding::Exact` returns for a split that does not divide evenly. The function surface diverges — this proposal's generic `dust_split`/`dust_split_into`/`dust_remainder` became per-kind `money_dust_split`/`qty_dust_split` (plus `_into` and `_remainder` variants each), with `parts` resolved as a plain `usize` equal-share count and no `price_dust_split` shipped — see [`exact_dust`'s surface decision](../../module/exact_dust/docs/decisions/002_equal_count_split_surface.md) and [leftover-reconstruction decision](../../module/exact_dust/docs/decisions/003_leftover_via_raw_minor_reconstruction.md) for the full account. The `_into` variants write each slot straight into the caller's buffer and make no heap allocation, as this listing requires.

### Enums

- `DustTo { First, Sink, Reject }`

### Functions

- `dust_split(total, parts, mode, to) -> Result<Vec-or-outbuf, DustError>` — the general split.
- `dust_remainder` — remainder access.
- `dust_split_into(out: &mut [Money])` — preferred hot-path variant; does not allocate.

### Errors

- `DustError { EmptyParts, Remainder, Overflow }`

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:469-474` | `exact_dust`'s full enum/function/error listing, including the non-allocating preference |
