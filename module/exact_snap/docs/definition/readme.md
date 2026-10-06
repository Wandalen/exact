# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `SnapError` | enum | `src/lib.rs:29` | [Grid Spacing Is Never Zero](../invariant/001_grid_spacing_never_zero.md) |
| `SnapError`'s `Display` impl | trait impl | `src/lib.rs:39` | [Grid Spacing Is Never Zero](../invariant/001_grid_spacing_never_zero.md) |
| `Tick` | struct | `src/lib.rs:70` | [Grid Spacing Is Never Zero](../invariant/001_grid_spacing_never_zero.md) |
| `Tick::new` | fn | `src/lib.rs:80` | [Grid Spacing Is Never Zero](../invariant/001_grid_spacing_never_zero.md) |
| `Tick::price` | fn | `src/lib.rs:96` | — |
| `Lot` | struct | `src/lib.rs:104` | [Grid Spacing Is Never Zero](../invariant/001_grid_spacing_never_zero.md) |
| `Lot::new` | fn | `src/lib.rs:113` | [Grid Spacing Is Never Zero](../invariant/001_grid_spacing_never_zero.md) |
| `Lot::qty` | fn | `src/lib.rs:124` | — |
| `price_snap_tick` | fn | `src/lib.rs:136` | — |
| `qty_snap_lot` | fn | `src/lib.rs:160` | — |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
