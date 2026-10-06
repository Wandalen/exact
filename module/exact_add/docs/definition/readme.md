# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `money_add` | fn | `src/lib.rs:53` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `money_sub` | fn | `src/lib.rs:63` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `qty_add` | fn | `src/lib.rs:73` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `qty_sub` | fn | `src/lib.rs:83` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `price_add` | fn | `src/lib.rs:93` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `price_sub` | fn | `src/lib.rs:103` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `money_checked_neg` | fn | `src/lib.rs:113` | [Reuse `KindError` Directly, No Wrapper Type](../decisions/001_reuse_kinderror_no_wrapper_type.md) |
| `money_saturating_add` | fn | `src/lib.rs:128` | — |
| `qty_saturating_add` | fn | `src/lib.rs:142` | — |

`money_saturating_add` and `qty_saturating_add` carry their own clamp-direction
correctness argument in their doc comments rather than a dedicated doc
instance — genuinely nothing in this crate's `docs/` covers them more
specifically than that, so this column stays honest with `—` rather than
forcing a link to the two decisions above, which are about error-type shape,
not about saturating clamp direction.

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
