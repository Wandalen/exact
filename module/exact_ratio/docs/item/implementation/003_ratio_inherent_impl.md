# 003: impl Ratio

## Representation

The inherent impl block carrying `Ratio`'s read access: 2 associated
functions/methods (`n`, `d`), one per private field. Each has its own
catalog entry under `associated_function/` — this entry documents the block
itself.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_ratio/src/lib.rs:102`

```rust
impl Ratio
{
  // 2 associated functions/methods — see associated_function/002-003.
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 102-117 | Declaration — the block spans both `Ratio` accessor members |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Declares `Ratio`'s read-accessor surface |
