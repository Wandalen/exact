# 005: impl< const SCALE: u32 > Qty< SCALE >

## Representation

The inherent impl block carrying `Qty`'s entire API: 3 associated constants
(`ZERO`, `EPSILON`, `MAX`) and 10 associated functions/methods
(`from_decimal`, `from_minor`, `from_int`, `as_decimal`, `minor`, `whole`,
`checked_add`, `checked_sub`, `checked_mul_int`, `parse`). Each member has
its own catalog entry — this entry documents the block itself.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:425`

```rust
impl< const SCALE : u32 > Qty< SCALE >
{
  // 3 associated constants, 10 associated functions/methods — see
  // associated_constant/006-008 and associated_function/010-019.
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 425-558 | Declaration — the block spans every `Qty` member |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declares the entire `Qty` API surface |
