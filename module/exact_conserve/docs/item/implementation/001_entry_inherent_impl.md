# 001: Entry inherent impl

## Representation

The one hand-written impl on `Entry`, providing its constructor.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:106-114`

```rust
impl Entry
{
  pub fn new( account : impl Into< String >, amount_minor : i64 ) -> Self
  {
    Self { account : account.into(), amount_minor }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 106-114 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | The sole constructor for `Entry`; see [new for Entry](../associated_function/001_new_entry.md) for its heavily-used call-site evidence |
