# 001: Entry inherent impl

## Representation

The one hand-written impl on `Entry< A >`, providing its constructor for any
asset key type `A`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:107-115`

```rust
impl< A > Entry< A >
{
  pub fn new( account : impl Into< String >, asset : A, amount_minor : i64 ) -> Self
  {
    Self { account : account.into(), asset, amount_minor }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 107-115 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | The sole constructor for `Entry`; see [new for Entry](../associated_function/001_new_entry.md) for its heavily-used call-site evidence |
