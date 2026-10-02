# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `money_add` | fn | `src/lib.rs:35` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `money_sub` | fn | `src/lib.rs:45` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `qty_add` | fn | `src/lib.rs:55` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `qty_sub` | fn | `src/lib.rs:65` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `price_add` | fn | `src/lib.rs:75` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `price_sub` | fn | `src/lib.rs:85` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `money_checked_neg` | fn | `src/lib.rs:95` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `money_saturating_add` | fn | `src/lib.rs:110` | — |
| `qty_saturating_add` | fn | `src/lib.rs:124` | — |

`money_saturating_add` and `qty_saturating_add` carry their own clamp-direction
correctness argument in their doc comments rather than a dedicated doc
instance — genuinely nothing in this crate's `docs/` covers them more
specifically than that, so this column stays honest with `—` rather than
forcing a link to the two decisions above, which are about error-type shape,
not about saturating clamp direction.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
