# 003: impl< const SCALE: u32 > Decimal< SCALE >

## Representation

The inherent impl block carrying `Decimal`'s entire API: 5 associated
constants (`ONE_MINOR`, `ZERO`, `EPSILON`, `MAX`, `MIN`) and 9 associated
functions/methods (`from_minor`, `from_int`, `minor`, `whole`, `checked_add`,
`checked_sub`, `checked_mul_int`, `checked_neg`, `parse`). Each member has
its own catalog entry under `associated_constant/` and `associated_function/`
— this entry documents the block itself.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:158`

```rust
impl< const SCALE : u32 > Decimal< SCALE >
{
  // 5 associated constants, 9 associated functions/methods — see
  // associated_constant/001-005 and associated_function/001-009.
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 158-359 | Declaration — the block spans every `Decimal` member |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declares the entire `Decimal` API surface |
